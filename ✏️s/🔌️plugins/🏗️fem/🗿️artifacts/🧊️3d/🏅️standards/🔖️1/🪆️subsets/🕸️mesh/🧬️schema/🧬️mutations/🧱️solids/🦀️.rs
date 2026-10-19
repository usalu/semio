//! 🧱️ `s.fem.fem3d` mesh mutations of solids (create, delete, replace, change).

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
                        pub mod create_solid {
                            #[path = "../🧊️create-solid/🦀️.rs"]
                            mod component;
                            #[path = "../🧊️create-solid/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🧊️create-solid/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod delete_solid {
                            #[path = "../🚫️delete-solid/🦀️.rs"]
                            mod component;
                            #[path = "../🚫️delete-solid/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🚫️delete-solid/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod replace_solid {
                            #[path = "../🔄️replace-solid/🦀️.rs"]
                            mod component;
                            #[path = "../🔄️replace-solid/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🔄️replace-solid/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_solid {
                            #[path = "../🩹️change-solid/🦀️.rs"]
                            mod component;
                            #[path = "../🩹️change-solid/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🩹️change-solid/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
}
                }
            }
        }
    }
}
