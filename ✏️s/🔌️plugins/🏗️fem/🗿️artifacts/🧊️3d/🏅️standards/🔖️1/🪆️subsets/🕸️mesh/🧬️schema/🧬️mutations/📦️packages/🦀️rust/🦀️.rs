//! 🕸️ `s.fem.fem3d` mesh mutations of sections and solids, and the selection move.

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
                        pub mod create_solid {
                            #[path = "../../🧊️create-solid/🦀️.rs"]
                            mod component;
                            #[path = "../../🧊️create-solid/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🧊️create-solid/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod delete_solid {
                            #[path = "../../🚫️delete-solid/🦀️.rs"]
                            mod component;
                            #[path = "../../🚫️delete-solid/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🚫️delete-solid/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod replace_solid {
                            #[path = "../../🔄️replace-solid/🦀️.rs"]
                            mod component;
                            #[path = "../../🔄️replace-solid/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🔄️replace-solid/↩️inverse/🦀️.rs"]
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
