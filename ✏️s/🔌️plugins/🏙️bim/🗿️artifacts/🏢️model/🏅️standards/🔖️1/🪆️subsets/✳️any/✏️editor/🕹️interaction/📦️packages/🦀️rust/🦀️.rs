//! 🕹️ interaction of `semio-s-artifact-bim-model` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as protocol;
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
                #[path = "../../../🧩️entities/🧬️families/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[cfg(test)]
            #[path = "../../../🧩️entities/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
        #[path = "."]
        pub mod interaction {
            #[path = "../../🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod schedule {
                        #[path = "../../../🎭️modes/✏️edit/🪟️windows/🧮️schedule/🏷️vocabulary/🦀️.rs"]
                        pub mod vocabulary;
                    }
                }
            }
        }
    }
}
