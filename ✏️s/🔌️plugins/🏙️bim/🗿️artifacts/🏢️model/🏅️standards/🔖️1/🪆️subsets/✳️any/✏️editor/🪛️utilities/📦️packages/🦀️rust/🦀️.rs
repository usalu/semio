//! 🪛️ BIM utility registry crate: the tools a window arms and the window kind ids they address.

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod bim {
        #[path = "."]
        pub mod utilities {
            #[path = "../../🦀️.rs"]
            mod component;
            pub use component::*;
        }
    }
}
