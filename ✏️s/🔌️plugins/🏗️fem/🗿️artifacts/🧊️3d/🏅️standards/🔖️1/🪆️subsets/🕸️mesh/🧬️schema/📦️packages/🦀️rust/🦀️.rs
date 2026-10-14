//! 🕸️ `s.fem.fem3d` mesh mutations of nodes and elements (create, delete, replace).

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
                        pub mod create_node {
                            #[path = "../../🧬️mutations/⚪️create-node/🦀️.rs"]
                            mod component;
                            #[path = "../../🧬️mutations/⚪️create-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🧬️mutations/⚪️create-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod delete_node {
                            #[path = "../../🧬️mutations/🕳️delete-node/🦀️.rs"]
                            mod component;
                            #[path = "../../🧬️mutations/🕳️delete-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🧬️mutations/🕳️delete-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod create_element {
                            #[path = "../../🧬️mutations/🧩️create-element/🦀️.rs"]
                            mod component;
                            #[path = "../../🧬️mutations/🧩️create-element/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🧬️mutations/🧩️create-element/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod delete_element {
                            #[path = "../../🧬️mutations/🗑️delete-element/🦀️.rs"]
                            mod component;
                            #[path = "../../🧬️mutations/🗑️delete-element/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🧬️mutations/🗑️delete-element/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod replace_element {
                            #[path = "../../🧬️mutations/♻️replace-element/🦀️.rs"]
                            mod component;
                            #[path = "../../🧬️mutations/♻️replace-element/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🧬️mutations/♻️replace-element/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod replace_node {
                            #[path = "../../🧬️mutations/🔁️replace-node/🦀️.rs"]
                            mod component;
                            #[path = "../../🧬️mutations/🔁️replace-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🧬️mutations/🔁️replace-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                    }
                }
            }
        }
    }
}
