// build.rs

#[allow(unused_imports)]
use std::io::ErrorKind;
use std::path::Path;
use std::{env, fs};

fn choose_source_dir() -> Option<String> {
    // Follow the 'recommended' install path
    if let Ok(path) = env::var("NDI_RUNTIME_DIR_V5") {
        if Path::new(&path).exists() {
            return Some(path);
        }
    }

    // Try the local lib folder
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = Path::new(&dir).join("lib");
    if path.exists() {
        return path.to_str().map(|s| s.to_string());
    }

    None
}

#[cfg(target_os = "windows")]
fn main() {
    let source_dir = choose_source_dir();

    // Copy the .dll/.lib files to the deps folder, to make it build
    if let Some(path) = source_dir {
        let source_path = Path::new(&path);
        let dest_path = Path::new(&env::var("OUT_DIR").unwrap()).join("../../../deps");
        fs::copy(
            source_path.join("..\\..\\NDI 5 SDK\\Lib\\x64\\Processing.NDI.Lib.x64.lib"),
            dest_path.join("Processing.NDI.Lib.x64.lib"),
        )
        .expect("copy Processing.NDI.Lib.x64.lib");
    }

    println!("cargo:rustc-link-lib=Processing.NDI.Lib.x64");
}

#[cfg(target_os = "macos")]
fn main() {

    let path = "/Library/NDI SDK for Apple/lib/macOS";

        let source_path = Path::new(&path);
        let dest_path = Path::new(&env::var("OUT_DIR").unwrap()).join("../../../deps");

        let source_file = source_path.join("libndi.dylib");
        let dest_file = dest_path.join("libndi.dylib");
        let s = &format!("copy libndi.dylib '{}' '{}'", source_file.to_str().unwrap(), dest_file.to_str().unwrap());

        fs::copy(source_file, dest_file).expect(s);

//        let sl_res = std::os::unix::fs::symlink(Path::new("libndi.so.3"), dest_path.join("libndi.so"));
//        if let Err(e) = sl_res {
//            if e.kind() != ErrorKind::AlreadyExists {
//                panic!("Unknown error: {}", e);
//            }
//        }


     if cfg!(not(feature = "dynamic-link")) {
         println!("cargo:rustc-link-lib=ndi");
     }
}


#[cfg(target_os = "linux")]
fn main() {
    let source_dir = choose_source_dir();

    // Copy the .so files to the deps folder, to make it build
    if let Some(path) = source_dir {
        let source_path = Path::new(&path);
        let dest_path = Path::new(&env::var("OUT_DIR").unwrap()).join("../../../deps");
        fs::copy(source_path.join("libndi.so.3"), dest_path.join("libndi.so.3")).expect("copy libndi.so.3");

        let sl_res = std::os::unix::fs::symlink(Path::new("libndi.so.3"), dest_path.join("libndi.so"));
        if let Err(e) = sl_res {
            if e.kind() != ErrorKind::AlreadyExists {
                panic!("Unknown error: {}", e);
            }
        }
    }

    if cfg!(not(feature = "dynamic-link")) {
        // Static link against it
        println!("cargo:rustc-link-lib=ndi");
    }
}
