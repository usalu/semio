//! 🚪️ io of `semio-s-artifact-bim-model` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                
                #[path = "."]
                pub mod io {
                    #[path = "../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}
