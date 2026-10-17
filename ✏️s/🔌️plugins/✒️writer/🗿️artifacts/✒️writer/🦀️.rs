//! ✒️ Writer artifact — the document entity this plugin's app edits.

#[path = "🤖️generated/📇️registry/🦀️.rs"]
pub mod graph_manifest;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

use semio_s_artifact_stdio_semio_document::schema::snapshot::{DocBlock, SemioDocumentSnapshot};
use std::sync::Arc;

pub use semio_s_artifact_writer_writer_schema::{WRITER_DIALECT, WRITER_DOCUMENT_SCHEMA};

pub use semio_s_artifact_writer_writer_schema::{default_uri};

pub use semio_s_artifact_writer_writer_schema::{WriterDocumentChild, document_child_handle, document_snapshot_from_text, text_from_document_snapshot};

pub use semio_s_artifact_writer_writer_schema::{WriterWorkingScene, attach_writer_document_text, document_child_handle_with_text, genesis_writer_child_pack, writer_snapshot_with_text, writer_text, writer_text_for_handle, writer_text_owner};

pub use semio_s_artifact_writer_writer_tests_2::{artifact_kind};

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
/// 🧪️ The mounted-app harness (`context`) of the writer editor: it drives the whole app, so it sits above every layer and the tests of commands, windows and panels reach it by this path.
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"]
pub mod editor_unit_tests;
//#endregion 🧪️Tests
pub use semio_s_artifact_writer_writer_v1::{WriterApplication, artifact, definition};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        pub use semio_s_artifact_writer_writer_v1::standards::v1::*;

        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                pub use semio_s_artifact_writer_writer_v1::standards::v1::subsets::any::*;

                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_writer_writer_schema::standards::v1::subsets::any::schema::*;

                    pub use semio_s_artifact_writer_writer_schema::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_writer_writer_operations::standards::v1::subsets::any::schema::operations;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_writer_writer_operations::standards::v1::subsets::any::schema::mutations::*;

                        #[path = "."]
                        pub mod rename_writer {
                            pub use semio_s_artifact_writer_writer_rename_writer::standards::v1::subsets::any::schema::mutations::rename_writer::*;

                            pub use semio_s_artifact_writer_writer_rename_writer::standards::v1::subsets::any::schema::mutations::rename_writer::diff;
                            pub use semio_s_artifact_writer_writer_rename_writer::standards::v1::subsets::any::schema::mutations::rename_writer::inverse;

                        }
                        #[path = "."]
                        pub mod change_uri {
                            pub use semio_s_artifact_writer_writer_rename_writer::standards::v1::subsets::any::schema::mutations::change_uri::*;

                            pub use semio_s_artifact_writer_writer_rename_writer::standards::v1::subsets::any::schema::mutations::change_uri::diff;
                            pub use semio_s_artifact_writer_writer_rename_writer::standards::v1::subsets::any::schema::mutations::change_uri::inverse;

                        }
                        #[path = "."]
                        pub mod change_language {
                            pub use semio_s_artifact_writer_writer_rename_writer::standards::v1::subsets::any::schema::mutations::change_language::*;

                            pub use semio_s_artifact_writer_writer_rename_writer::standards::v1::subsets::any::schema::mutations::change_language::diff;
                            pub use semio_s_artifact_writer_writer_rename_writer::standards::v1::subsets::any::schema::mutations::change_language::inverse;

                        }
                        #[path = "."]
                        pub mod edit_text {
                            pub use semio_s_artifact_writer_writer_rename_writer::standards::v1::subsets::any::schema::mutations::edit_text::*;

                            pub use semio_s_artifact_writer_writer_rename_writer::standards::v1::subsets::any::schema::mutations::edit_text::diff;
                            pub use semio_s_artifact_writer_writer_rename_writer::standards::v1::subsets::any::schema::mutations::edit_text::inverse;

                        }
                        #[path = "."]
                        pub mod splice_text {
                            pub use semio_s_artifact_writer_writer_rename_writer::standards::v1::subsets::any::schema::mutations::splice_text::*;

                            pub use semio_s_artifact_writer_writer_rename_writer::standards::v1::subsets::any::schema::mutations::splice_text::diff;
                            pub use semio_s_artifact_writer_writer_rename_writer::standards::v1::subsets::any::schema::mutations::splice_text::inverse;

                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_writer_writer_v1::standards::v1::subsets::any::io::*;
pub mod binary {
    pub use semio_s_artifact_writer_writer_v1::standards::v1::subsets::any::io::binary::*;
    pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::binary::diff;
    pub use semio_s_artifact_writer_writer_tests::standards::v1::subsets::any::io::binary::inferences;
    pub use semio_s_artifact_writer_writer_schema::standards::v1::subsets::any::io::binary::snapshot;
}
pub mod sqlite {
    pub use semio_s_artifact_writer_writer_tests::standards::v1::subsets::any::io::sqlite::*;
    pub use semio_s_artifact_writer_writer_schema::standards::v1::subsets::any::io::sqlite::snapshot;
}
pub mod text {
    pub use semio_s_artifact_writer_writer_v1::standards::v1::subsets::any::io::text::*;
    pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::text::diff;
    pub use semio_s_artifact_writer_writer_operations::standards::v1::subsets::any::io::text::mutations;
}

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

// ---- Artifact root facets ----
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}

pub mod op {
    pub use crate::standards::v1::subsets::any::schema::mutations::WriterMutation;
}

pub use semio_s_artifact_writer_writer_inferences::diff;
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub use semio_s_artifact_writer_writer_inferences::snapshot;
pub use crate::editor::writer::modes::edit::windows::main::config::{default_camera, default_zoom, WriterCamera, WriterEditorSettings};
pub use crate::editor::writer::modes::edit::windows::main::transient::WriterEditorSelection;
pub use crate::standards::v1::subsets::any::schema::diff::WriterDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::WriterMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::WriterSnapshot;

pub use semio_s_artifact_writer_writer_tests::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod writer {
        pub use semio_s_artifact_writer_writer_tests_2::editor::writer::*;
pub use semio_s_artifact_writer_writer_tests::editor::writer::demo_session;

        pub use semio_s_artifact_writer_writer_inferences::editor::writer::base;
        pub use semio_s_artifact_writer_writer_tests::editor::writer::terminology;
        pub use semio_s_artifact_writer_writer_tests::editor::writer::wasm;

        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_writer_writer_operations::editor::writer::commands::commit_rename;
            pub use semio_s_artifact_writer_writer_operations::editor::writer::commands::engagement_input;
            pub use semio_s_artifact_writer_writer_tests_2::editor::writer::commands::engagement_submit;
            pub use semio_s_artifact_writer_writer_tests_2::editor::writer::commands::format_document;
            pub use semio_s_artifact_writer_writer_tests_2::editor::writer::commands::lint_document;
            pub use semio_s_artifact_writer_writer_tests_2::editor::writer::commands::open_document;
            pub use semio_s_artifact_writer_writer_tests_2::editor::writer::commands::request_completions;
            pub use semio_s_artifact_writer_writer_tests_2::editor::writer::commands::set_active_example;
            pub use semio_s_artifact_writer_writer_tests_2::editor::writer::commands::set_camera;
            pub use semio_s_artifact_writer_writer_tests_2::editor::writer::commands::set_editor_selection;
            pub use semio_s_artifact_writer_writer_tests_2::editor::writer::commands::load_document_json;
            pub use semio_s_artifact_writer_writer_tests_2::editor::writer::commands::set_font_px;
            pub use semio_s_artifact_writer_writer_tests_2::editor::writer::commands::set_line_height;
            pub use semio_s_artifact_writer_writer_tests_2::editor::writer::commands::set_tab_size;
            pub use semio_s_artifact_writer_writer_tests_2::editor::writer::commands::set_text;
            pub use semio_s_artifact_writer_writer_tests_2::editor::writer::commands::text_edit;
            pub use semio_s_artifact_writer_writer_tests_2::editor::writer::commands::text_splice;
            pub use semio_s_artifact_writer_writer_tests_2::editor::writer::commands::toggle_line_numbers;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_writer_writer_tests_2::editor::writer::modes::edit::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_writer_writer_editor::editor::writer::modes::edit::windows::main::*;

                        pub use semio_s_artifact_writer_writer_editor::editor::writer::modes::edit::windows::main::options;
                    }
                }
            }
        }

        #[path = "."]
        pub mod panels {
            pub use semio_s_artifact_writer_writer_operations::editor::writer::panels::catalogue;
            pub use semio_s_artifact_writer_writer_operations::editor::writer::panels::document;
            pub use semio_s_artifact_writer_writer_operations::editor::writer::panels::inspection;
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod writer {
        pub use semio_s_artifact_writer_writer_tests_2::viewer::writer::*;

        pub use semio_s_artifact_writer_writer_tests::viewer::writer::presence;
        pub use semio_s_artifact_writer_writer_tests::viewer::writer::config;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_writer_writer_operations::viewer::writer::modes::view::*;

                pub use semio_s_artifact_writer_writer_inferences::viewer::writer::modes::view::windows;
            }
        }
    }
}

pub use semio_s_artifact_writer_writer_tests_2::host;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_0;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_1;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_2;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_3;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
mod hoisted_tests_4;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/✂️concurrent-typing/🦀️.rs"]
mod hoisted_tests_5;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️typing-runs/🦀️.rs"]
mod hoisted_tests_6;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️engagement-submit/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_7;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔍️lint-document/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_8;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎥️set-camera/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_9;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📝️text-edit/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_10;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✂️text-splice/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_11;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔢️toggle-line-numbers/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_12;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_13;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_14;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_15;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_16;
