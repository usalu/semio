//! 🧬️ mut-any of `semio-s-artifact-architect-program` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;

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
                    pub use semio_s_artifact_architect_program_tests::standards::v1::subsets::any::schema::kernel;
                    #[path = "."]
                    pub mod registers {
                        pub use semio_s_artifact_architect_program_registers::standards::v1::subsets::any::schema::registers::*;
                    }
                    pub use semio_s_artifact_architect_program_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_architect_program_inferences::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod replace_governance {
                            #[path = "../../🏛️governance/♻️replace/🦀️.rs"]
                            mod component;
                            #[path = "../../🏛️governance/♻️replace/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🏛️governance/♻️replace/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod rename_meta {
                            #[path = "../../🏷️meta/🏷️rename/🦀️.rs"]
                            mod component;
                            #[path = "../../🏷️meta/🏷️rename/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🏷️meta/🏷️rename/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod replace_meta {
                            #[path = "../../🏷️meta/♻️replace/🦀️.rs"]
                            mod component;
                            #[path = "../../🏷️meta/♻️replace/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🏷️meta/♻️replace/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod rename_project {
                            #[path = "../../🏙️project/🏷️rename/🦀️.rs"]
                            mod component;
                            #[path = "../../🏙️project/🏷️rename/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🏙️project/🏷️rename/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod replace_project {
                            #[path = "../../🏙️project/♻️replace/🦀️.rs"]
                            mod component;
                            #[path = "../../🏙️project/♻️replace/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🏙️project/♻️replace/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod connect_adjacency {
                            #[path = "../../🧲️adjacency/🧲️connect/🦀️.rs"]
                            mod component;
                            #[path = "../../🧲️adjacency/🧲️connect/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🧲️adjacency/🧲️connect/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod connect_trace {
                            #[path = "../../🧵️trace/🧵️connect/🦀️.rs"]
                            mod component;
                            #[path = "../../🧵️trace/🧵️connect/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🧵️trace/🧵️connect/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod disconnect_trace {
                            #[path = "../../🧵️trace/✂️disconnect/🦀️.rs"]
                            mod component;
                            #[path = "../../🧵️trace/✂️disconnect/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🧵️trace/✂️disconnect/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod disconnect_adjacency {
                            #[path = "../../🧲️adjacency/🫷️disconnect/🦀️.rs"]
                            mod component;
                            #[path = "../../🧲️adjacency/🫷️disconnect/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🧲️adjacency/🫷️disconnect/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                    }
                }
            }
        }
    }
}
