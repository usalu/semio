//! 🕸️ `s.fem.fem2d` mesh mutations of sections and regions, and the selection move.

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
                            #[path = "../../📐️create-section/🦀️.rs"]
                            mod component;
                            #[path = "../../📐️create-section/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../📐️create-section/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod delete_section {
                            #[path = "../../✂️delete-section/🦀️.rs"]
                            mod component;
                            #[path = "../../✂️delete-section/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../✂️delete-section/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod replace_section {
                            #[path = "../../📏️replace-section/🦀️.rs"]
                            mod component;
                            #[path = "../../📏️replace-section/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../📏️replace-section/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod create_region {
                            #[path = "../../🗺️create-region/🦀️.rs"]
                            mod component;
                            #[path = "../../🗺️create-region/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🗺️create-region/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod delete_region {
                            #[path = "../../🚫️delete-region/🦀️.rs"]
                            mod component;
                            #[path = "../../🚫️delete-region/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🚫️delete-region/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod replace_region {
                            #[path = "../../🔄️replace-region/🦀️.rs"]
                            mod component;
                            #[path = "../../🔄️replace-region/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🔄️replace-region/↩️inverse/🦀️.rs"]
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
