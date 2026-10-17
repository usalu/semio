//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

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
                    pub use semio_s_artifact_writer_writer_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_writer_writer_schema::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod operations {
                        pub use semio_s_artifact_writer_writer_operations::standards::v1::subsets::any::schema::operations::*;
                        #[cfg(test)]
                        #[path = "../../../🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_writer_writer_operations::standards::v1::subsets::any::schema::mutations::*;
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
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_writer_writer_operations::standards::v1::subsets::any::io::binary::mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_writer_writer_schema::standards::v1::subsets::any::io::text::snapshot;
                        #[path = "."]
                        pub mod mutations {
                            pub use semio_s_artifact_writer_writer_operations::standards::v1::subsets::any::io::text::mutations::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🚪️io/📝️text/🧬️mutations/🧪️tests/🔬️semio-grammar-conformance/🦀️.rs"]
                            pub mod semio_grammar_conformance;
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
        #[path = "../../../../../../../✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        pub use semio_s_artifact_writer_writer_inferences::editor::writer::base;
        pub use semio_s_artifact_writer_writer_tests::editor::writer::terminology;
        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_writer_writer_operations::editor::writer::commands::commit_rename;
            pub use semio_s_artifact_writer_writer_operations::editor::writer::commands::engagement_input;
            #[path = "../../../../../../../✏️editor/🎮️commands/📤️engagement-submit/🦀️.rs"]
            pub mod engagement_submit;
            #[path = "../../../../../../../✏️editor/🎮️commands/🧹️format-document/🦀️.rs"]
            pub mod format_document;
            #[path = "../../../../../../../✏️editor/🎮️commands/🔍️lint-document/🦀️.rs"]
            pub mod lint_document;
            #[path = "../../../../../../../✏️editor/🎮️commands/📖️open-document/🦀️.rs"]
            pub mod open_document;
            #[path = "../../../../../../../✏️editor/🎮️commands/✨️request-completions/🦀️.rs"]
            pub mod request_completions;
            #[path = "../../../../../../../✏️editor/🎮️commands/🧺️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "../../../../../../../✏️editor/🎮️commands/🎥️set-camera/🦀️.rs"]
            pub mod set_camera;
            #[path = "../../../../../../../✏️editor/🎮️commands/🗂️set-editor-selection/🦀️.rs"]
            pub mod set_editor_selection;
            #[path = "../../../../../../../✏️editor/🎮️commands/📄️load-document-json/🦀️.rs"]
            pub mod load_document_json;
            #[path = "../../../../../../../✏️editor/🎮️commands/⚙️set-font-px/🦀️.rs"]
            pub mod set_font_px;
            #[path = "../../../../../../../✏️editor/🎮️commands/📏️set-line-height/🦀️.rs"]
            pub mod set_line_height;
            #[path = "../../../../../../../✏️editor/🎮️commands/📐️set-tab-size/🦀️.rs"]
            pub mod set_tab_size;
            #[path = "../../../../../../../✏️editor/🎮️commands/🔤️set-text/🦀️.rs"]
            pub mod set_text;
            #[path = "../../../../../../../✏️editor/🎮️commands/📝️text-edit/🦀️.rs"]
            pub mod text_edit;
            #[path = "../../../../../../../✏️editor/🎮️commands/✂️text-splice/🦀️.rs"]
            pub mod text_splice;
            #[path = "../../../../../../../✏️editor/🎮️commands/🔢️toggle-line-numbers/🦀️.rs"]
            pub mod toggle_line_numbers;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_writer_writer_editor::editor::writer::modes::edit::windows::main::*;
                        #[path = "."]
                        pub mod transient {
                            pub use semio_s_artifact_writer_writer_editor::editor::writer::modes::edit::windows::main::transient::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient/🧪️tests/🧩️partial/🦀️.rs"]
                            pub mod preparation_tests;
                        }
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
        #[path = "../../../../../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
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
#[path = "."]
pub mod host {
#[path = "../../../../../../../../../../../🔨️modules/🏠️host/🧰️owned/🦀️.rs"]
pub mod owned;
}
