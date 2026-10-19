//! 🕸️ `s.fem.fem3d` mesh mutations of node changes and the selection move.

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
                        pub mod change_node {
                            #[path = "../../🩹️change-node/🦀️.rs"]
                            mod component;
                            #[path = "../../🩹️change-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🩹️change-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod move_selection {
                            #[path = "../../🧭️move-selection/🦀️.rs"]
                            mod component;
                            #[path = "../../🧭️move-selection/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🧭️move-selection/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                    }
                }
            }
        }
    }
}
