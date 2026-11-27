//! 🔬️ entities-families-tests of `semio-s-artifact-bim-model`: the entity table tests, carved out of the interaction crate.

extern crate semio_os_kernel as protocol;
extern crate semio_framework_schema as schema;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod bim {
        #[path = "."]
        pub mod entities {
            #[path = "."]
            pub mod families {
                #[cfg(test)]
                #[path = "../../🦀️.rs"]
                pub mod tests;
            }
        }
    }
}
