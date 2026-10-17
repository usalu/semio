//! 🔖️ v1 of `semio-s-artifact-writer-writer` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::standards::v1::subsets::any::schema::mutations::WriterMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::WriterSnapshot;
#[cfg(test)]
pub use crate::editor::writer::modes::edit::windows::main::config::WriterCamera;
#[cfg(test)]
pub use crate::editor::writer::modes::edit::windows::main::transient::WriterEditorSelection;
pub use semio_s_artifact_writer_writer_schema::{WRITER_DIALECT, WRITER_DOCUMENT_SCHEMA};
pub use crate::standards::v1::artifact_items_6::{WriterApplication, artifact, definition};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "../../🪆️subsets/✳️any/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_writer_writer_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_writer_writer_schema::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::schema::inferences;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_writer_writer_operations::standards::v1::subsets::any::schema::mutations::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::pdf;
                                pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::docx;
                                pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::md;
                                pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::txt;
                                pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::pdf;
                                pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::docx;
                                pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::md;
                                pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                            }
                        }
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_writer_writer_tests::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod writer {
        pub use semio_s_artifact_writer_writer_tests_2::editor::writer::*;
        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_writer_writer_operations::editor::writer::commands::engagement_input;
            pub use semio_s_artifact_writer_writer_tests_2::editor::writer::commands::lint_document;
            pub use semio_s_artifact_writer_writer_tests_2::editor::writer::commands::set_camera;
            pub use semio_s_artifact_writer_writer_tests_2::editor::writer::commands::set_editor_selection;
            pub use semio_s_artifact_writer_writer_tests_2::editor::writer::commands::set_font_px;
        }
        #[path = "."]
        pub mod modes {
            pub use semio_s_artifact_writer_writer_tests_2::editor::writer::modes::*;
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_writer_writer_tests_2::editor::writer::modes::edit::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_writer_writer_editor::editor::writer::modes::edit::windows::main::*;
                        #[path = "."]
                        pub mod config {
                            pub use semio_s_artifact_writer_writer_editor::editor::writer::modes::edit::windows::main::config::*;
                            #[cfg(test)]
                            #[path = "../../🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🎚️config/🧪️tests/🔬️window-state/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod writer {
        pub use semio_s_artifact_writer_writer_tests_2::viewer::writer::*;
    }
}
