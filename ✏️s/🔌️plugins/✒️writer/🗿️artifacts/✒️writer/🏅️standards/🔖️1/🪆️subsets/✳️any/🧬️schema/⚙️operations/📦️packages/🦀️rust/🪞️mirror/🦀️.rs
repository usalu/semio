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
                    pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::schema::diff;
                    #[path = "../../../🦀️.rs"]
                    pub mod operations;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../../🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod rename_writer {
                            pub use semio_s_artifact_writer_writer_rename_writer::standards::v1::subsets::any::schema::mutations::rename_writer::*;
                            pub use semio_s_artifact_writer_writer_rename_writer::standards::v1::subsets::any::schema::mutations::rename_writer::diff;
                            pub use semio_s_artifact_writer_writer_rename_writer::standards::v1::subsets::any::schema::mutations::rename_writer::inverse;
                            #[cfg(test)]
                            #[path = "../../../../🧬️mutations/🏷️rename-writer/🧪️tests/🏷️renames/🦀️.rs"]
                            pub mod tests_renames_the_document_to_mission_brief;
                        }
                        #[path = "."]
                        pub mod change_uri {
                            pub use semio_s_artifact_writer_writer_rename_writer::standards::v1::subsets::any::schema::mutations::change_uri::*;
                            pub use semio_s_artifact_writer_writer_rename_writer::standards::v1::subsets::any::schema::mutations::change_uri::diff;
                            pub use semio_s_artifact_writer_writer_rename_writer::standards::v1::subsets::any::schema::mutations::change_uri::inverse;
                            #[cfg(test)]
                            #[path = "../../../../🧬️mutations/🔗change-uri/🧪️tests/🔗️republishes/🦀️.rs"]
                            pub mod tests_republishes_the_brief_under_a_new_uri;
                        }
                        #[path = "."]
                        pub mod change_language {
                            pub use semio_s_artifact_writer_writer_rename_writer::standards::v1::subsets::any::schema::mutations::change_language::*;
                            pub use semio_s_artifact_writer_writer_rename_writer::standards::v1::subsets::any::schema::mutations::change_language::diff;
                            pub use semio_s_artifact_writer_writer_rename_writer::standards::v1::subsets::any::schema::mutations::change_language::inverse;
                            #[cfg(test)]
                            #[path = "../../../../🧬️mutations/🌐change-language/🧪️tests/🔤️switches/🦀️.rs"]
                            pub mod tests_switches_the_brief_from_plaintext_to_markdown;
                        }
                        #[path = "."]
                        pub mod edit_text {
                            pub use semio_s_artifact_writer_writer_rename_writer::standards::v1::subsets::any::schema::mutations::edit_text::*;
                            pub use semio_s_artifact_writer_writer_rename_writer::standards::v1::subsets::any::schema::mutations::edit_text::diff;
                            pub use semio_s_artifact_writer_writer_rename_writer::standards::v1::subsets::any::schema::mutations::edit_text::inverse;
                            #[cfg(test)]
                            #[path = "../../../../🧬️mutations/✏️edit-text/🧪️tests/⚠️warns/🦀️.rs"]
                            pub mod tests_warns_that_the_brief_body_is_unchanged;
                        }
                        #[path = "."]
                        pub mod splice_text {
                            pub use semio_s_artifact_writer_writer_rename_writer::standards::v1::subsets::any::schema::mutations::splice_text::*;
                            pub use semio_s_artifact_writer_writer_rename_writer::standards::v1::subsets::any::schema::mutations::splice_text::diff;
                            pub use semio_s_artifact_writer_writer_rename_writer::standards::v1::subsets::any::schema::mutations::splice_text::inverse;
                            #[cfg(test)]
                            #[path = "../../../../🧬️mutations/✂️splice-text/🧪️tests/⚠️warns/🦀️.rs"]
                            pub mod tests_warns_that_an_already_removed_run_leaves_the_brief_unchanged;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
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
                                #[path = "."]
                                pub mod docx {
                                    pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::docx::*;
                                    #[path = "."]
                                    pub mod v_ecma_376 {
                                        pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::docx::v_ecma_376::*;
                                        #[path = "."]
                                        pub mod any {
                                            pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::docx::v_ecma_376::any::*;
                                            #[cfg(test)]
                                            #[path = "../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/📜️docx/🔖️ecma-376/✳️any/🧪️tests/🔬️unit/🦀️.rs"]
                                            pub mod tests;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod md {
                                    pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::md::*;
                                    #[path = "."]
                                    pub mod v_commonmark {
                                        pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::md::v_commonmark::*;
                                        #[path = "."]
                                        pub mod any {
                                            pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::md::v_commonmark::any::*;
                                            #[cfg(test)]
                                            #[path = "../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🧪️tests/🔬️unit/🦀️.rs"]
                                            pub mod tests;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::json::*;
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::json::v_rfc8259::*;
                                        #[path = "."]
                                        pub mod any {
                                            pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::json::v_rfc8259::any::*;
                                            #[cfg(test)]
                                            #[path = "../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🧪️tests/🔬️unit/🦀️.rs"]
                                            pub mod tests;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod writer {
        pub use semio_s_artifact_writer_writer_inferences::editor::writer::editor_vocabulary;
        pub use semio_s_artifact_writer_writer_tests::editor::writer::terminology;
        #[path = "."]
        pub mod commands {
            #[path = "../../../../../✏️editor/🎮️commands/✍️commit-rename/🦀️.rs"]
            pub mod commit_rename;
            #[path = "../../../../../✏️editor/🎮️commands/💬️engagement-input/🦀️.rs"]
            pub mod engagement_input;
        }
        #[path = "."]
        pub mod panels {
            #[path = "../../../../../✏️editor/📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "../../../../../✏️editor/📌️panels/🗿️artifact/🦀️.rs"]
            pub mod document;
            #[path = "../../../../../✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod writer {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_writer_writer_inferences::viewer::writer::modes::view::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_writer_writer_inferences::viewer::writer::modes::view::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/✒️main/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
    }
}
