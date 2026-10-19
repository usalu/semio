//! 🪚️ `s.fem.fem3d` mesh mutations of sections (create, delete, replace, change).

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
                        pub mod create_section {
                            #[path = "../📐️create-section/🦀️.rs"]
                            mod component;
                            #[path = "../📐️create-section/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../📐️create-section/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod delete_section {
                            #[path = "../✂️delete-section/🦀️.rs"]
                            mod component;
                            #[path = "../✂️delete-section/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../✂️delete-section/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod replace_section {
                            #[path = "../📏️replace-section/🦀️.rs"]
                            mod component;
                            #[path = "../📏️replace-section/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../📏️replace-section/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }

                        #[path = "."]
                        pub mod change_section {
                            #[path = "../🩹️change-section/🦀️.rs"]
                            mod component;
                            #[path = "../🩹️change-section/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🩹️change-section/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
}
                }
            }
        }
    }
}
