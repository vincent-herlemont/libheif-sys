fn main() {
    if std::env::var("DOCS_RS").is_ok() {
        // Don't link with libheif in case of building documentation for docs.rs.
        println!("cargo:rustc-cfg=docs_rs");
        return;
    }

    #[cfg(feature = "use-bindgen")]
    {
        println!("cargo:rustc-link-lib=static=heif");
        println!("cargo:rustc-link-search=native=/Users/vincentherlemont/Development/projects/jolypath_tests/docs_treatment/image/libheif/libheif/build/install/lib");
        // println!("cargo:rustc-link-lib=static=de265");
        // println!("cargo:rustc-link-search=native=/Users/vincentherlemont/Development/projects/jolypath_tests/docs_treatment/image/libheif/libde265/build/install/lib");
        
        // Import from brew webp
        //println!("cargo:rustc-link-lib=dylib=webp");
        // brew --prefix webm + lib
        //println!("cargo:rustc-link-search=native=/opt/homebrew/Cellar/webp/1.4.0/lib");

        // Import form brew libde265
        println!("cargo:rustc-link-lib=dylib=de265");
        // brew --prefix de265 + lib
        println!("cargo:rustc-link-search=native=/opt/homebrew/Cellar/libde265/1.0.15/lib");

        // Import from brew x265
        println!("cargo:rustc-link-lib=dylib=x265");
        // brew --prefix x265 + lib
        println!("cargo:rustc-link-search=native=/opt/homebrew/opt/x265/lib");

        // Import from brew aom
        println!("cargo:rustc-link-lib=dylib=aom");
        // brew --prefix aom + lib
        println!("cargo:rustc-link-search=native=/opt/homebrew/opt/aom/lib");

        // Import from brew /opt/homebrew/opt/libtool
        // println!("cargo:rustc-link-lib=dylib=ltdl");
        // brew --prefix libtool + lib
        // println!("cargo:rustc-link-search=native=/opt/homebrew/Cellar/libtool/2.4.7/lib");

        // Import from brew jpeg
        println!("cargo:rustc-link-lib=dylib=jpeg");
        // brew --prefix jpeg + lib
        println!("cargo:rustc-link-search=native=/opt/homebrew/opt/jpeg/lib");

        println!("cargo:rustc-link-lib=c++");

        println!("cargo:rerun-if-changed=wrapper.h");

        use std::env;
        use std::path::PathBuf;
        // The bindgen::Builder is the main entry point
        // to bindgen, and lets you build up options for
        // the resulting bindings.
        let mut builder = bindgen::Builder::default()
            // The input header we would like to generate
            // bindings for.
            .header("wrapper.h")
            .generate_comments(true)
            .generate_cstr(true)
            .ctypes_prefix("libc")
            .allowlist_function("heif_.*")
            .allowlist_type("heif_.*")
            .size_t_is_usize(true)
            .clang_args(&[
                "-fparse-all-comments",
                "-fretain-comments-from-system-headers",
                "-I/Users/vincentherlemont/Development/projects/jolypath_tests/docs_treatment/image/libheif/libheif/build/install/include",
            ]);

        // Finish the builder and generate the bindings.
        let bindings = builder
            .generate()
            // Unwrap the Result and panic on failure.
            .expect("Unable to generate bindings");

        // Write the bindings to the $OUT_DIR/bindings.rs file.
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }
}
