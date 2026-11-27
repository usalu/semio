//! 🧱️ BIM editor kit core: the UI admission helpers every taxonomy node shares, the id mint, and the window-config declaration macro.

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
        pub mod kit {
            #[path = "../../../🦀️.rs"]
            mod component;
            pub use component::*;
        }
    }
}
