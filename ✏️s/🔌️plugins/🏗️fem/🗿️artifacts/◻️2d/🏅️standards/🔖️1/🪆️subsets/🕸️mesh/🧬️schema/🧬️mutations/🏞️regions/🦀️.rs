//! 🏞️ `s.fem.fem2d` mesh mutations of regions (create, delete, replace, change).

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
                        pub mod create_region {
                            #[path = "../🗺️create-region/🦀️.rs"]
                            mod component;
                            #[path = "../🗺️create-region/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🗺️create-region/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod delete_region {
                            #[path = "../🚫️delete-region/🦀️.rs"]
                            mod component;
                            #[path = "../🚫️delete-region/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🚫️delete-region/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod replace_region {
                            #[path = "../🔄️replace-region/🦀️.rs"]
                            mod component;
                            #[path = "../🔄️replace-region/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🔄️replace-region/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }

                        #[path = "."]
                        pub mod change_region {
                            #[path = "../🩹️change-region/🦀️.rs"]
                            mod component;
                            #[path = "../🩹️change-region/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🩹️change-region/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
}
                }
            }
        }
    }
}
