//! 🧬️ schema of `semio-s-artifact-architect-program` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as framework_schema;

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
                    #[path = "../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_architect_program_tests::standards::v1::subsets::any::schema::kernel;
                    #[path = "."]
                    pub mod registers {
                        pub use semio_s_artifact_architect_program_registers::standards::v1::subsets::any::schema::registers::*;
                        pub use semio_s_artifact_architect_program_catalog::standards::v1::subsets::any::schema::registers::catalog;
                    }
                    pub use semio_s_artifact_architect_program_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_architect_program_inferences::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_architect_program_inferences::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod create_register_row {
                            #[path = "../../🧬️mutations/📋️register-row/🌱️create/🦀️.rs"]
                            mod component;
                            #[path = "../../🧬️mutations/📋️register-row/🌱️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🧬️mutations/📋️register-row/🌱️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod delete_register_row {
                            #[path = "../../🧬️mutations/📋️register-row/🗑️delete/🦀️.rs"]
                            mod component;
                            #[path = "../../🧬️mutations/📋️register-row/🗑️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🧬️mutations/📋️register-row/🗑️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod rename_register_row {
                            #[path = "../../🧬️mutations/📋️register-row/🏷️rename/🦀️.rs"]
                            mod component;
                            #[path = "../../🧬️mutations/📋️register-row/🏷️rename/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🧬️mutations/📋️register-row/🏷️rename/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod replace_register_row {
                            #[path = "../../🧬️mutations/📋️register-row/♻️replace/🦀️.rs"]
                            mod component;
                            #[path = "../../🧬️mutations/📋️register-row/♻️replace/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🧬️mutations/📋️register-row/♻️replace/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod architect {
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_architect_program_config::editor::architect::config::*;
            #[cfg(test)]
            #[path = "../../../✏️editor/🎚️config/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
            #[cfg(test)]
            #[path = "../../../✏️editor/🎚️config/🧪️tests/🔬️contract-vectors/🦀️.rs"]
            pub mod contract_vectors;
        }
        #[path = "."]
        pub mod chrome {
            pub use semio_s_artifact_architect_program_snapshot::editor::architect::chrome::*;
            #[cfg(test)]
            #[path = "../../../✏️editor/🎨️chrome/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod architect {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "../../../👁️viewer/🎭️modes/👁️view/🪟️windows/📋️register/🦀️.rs"]
                    pub mod register;
                }
            }
        }
    }
}
