//! 🔩️ `s.fem.fem2d` mesh mutations of element changes.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {

                        #[path = "."]
                        pub mod change_element {
                            #[path = "../🩹️change-element/🦀️.rs"]
                            mod component;
                            #[path = "../🩹️change-element/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🩹️change-element/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
}
                }
            }
        }
    }
}
