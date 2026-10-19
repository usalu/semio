//! 🧩️ entities of `semio-s-artifact-bim-model` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_schema as schema;


#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod bim {
        #[path = "."]
        pub mod entities {
            #[path = "../../🦀️.rs"]
            mod component;
            pub use component::*;
        }
        
    }
}
