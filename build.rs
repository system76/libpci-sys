extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=pci");

    let bindings = bindgen::Builder::default()
    .header("wrapper.h")
    .blacklist_type("u8")
    .blacklist_type("u16")
    .blacklist_type("u32")
    .blacklist_type("u64")
    .generate()
    .expect("failed to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("failed to write bindings");
}
