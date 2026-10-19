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
                    pub use semio_s_artifact_forms_forms_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_forms_forms_definition::standards::v1::subsets::any::schema::definition;
                    pub use semio_s_artifact_forms_forms_definition::standards::v1::subsets::any::schema::response;
                    pub use semio_s_artifact_forms_forms_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_forms_forms_mutations::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod discard_response {
                            pub use semio_s_artifact_forms_forms_mutations::standards::v1::subsets::any::schema::mutations::discard_response::mutation;
                        }
                        #[path = "."]
                        pub mod commit_response {
                            pub use semio_s_artifact_forms_forms_mutations::standards::v1::subsets::any::schema::mutations::commit_response::mutation;
                        }
                        #[path = "."]
                        pub mod move_block_to_step {
                            pub use semio_s_artifact_forms_forms_diff::standards::v1::subsets::any::schema::mutations::move_block_to_step::mutation;
                        }
                        #[path = "."]
                        pub mod reorder_step {
                            pub use semio_s_artifact_forms_forms_diff::standards::v1::subsets::any::schema::mutations::reorder_step::mutation;
                        }
                        #[path = "."]
                        pub mod create_block {
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::create_block::mutation;
                        }
                        #[path = "."]
                        pub mod create_step {
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::create_step::mutation;
                        }
                        #[path = "."]
                        pub mod delete_block {
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::delete_block::mutation;
                        }
                        #[path = "."]
                        pub mod delete_step {
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::delete_step::mutation;
                        }
                        #[path = "."]
                        pub mod change_form_title {
                            pub use semio_s_artifact_forms_forms_diff::standards::v1::subsets::any::schema::mutations::change_form_title::mutation;
                        }
                        #[path = "."]
                        pub mod change_step_description {
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::change_step_description::mutation;
                        }
                        #[path = "."]
                        pub mod rename_step {
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::rename_step::mutation;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_forms_forms_snapshot::standards::v1::subsets::any::io::text::snapshot::*;
                            #[path = "."]
                            pub mod response {
                                pub use semio_s_artifact_forms_forms_definition::standards::v1::subsets::any::io::text::snapshot::response::export;
                            }
                        }
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_forms_forms_mutations::op;
pub use semio_s_artifact_forms_forms_definition::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod forms {
        #[path = "."]
        pub mod questions {
            pub use semio_s_artifact_forms_forms_questions::editor::forms::questions::*;
            pub use semio_s_artifact_forms_forms_diff::editor::forms::questions::visibility;
            pub use semio_s_artifact_forms_forms_tests_3::editor::forms::questions::placement;
        }
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_forms_forms_config::editor::forms::config::*;
        }
        pub use semio_s_artifact_forms_forms_tests::editor::forms::support;
        #[path = "."]
        pub mod commands {
            #[path = "../../../🦀️.rs"]
            pub mod add_question;
            #[path = "../../../../🔘️add-question-option/🦀️.rs"]
            pub mod add_question_option;
            #[path = "../../../../📃️add-step/🦀️.rs"]
            pub mod add_step;
            #[path = "../../../../📐️add-vector-field/🦀️.rs"]
            pub mod add_vector_field;
            #[path = "../../../../🫳️drop-question-kind/🦀️.rs"]
            pub mod drop_question_kind;
            #[path = "../../../../📤️export-snapshot/🦀️.rs"]
            pub mod export_snapshot;
            #[path = "../../../../📤️export-responses/🦀️.rs"]
            pub mod export_responses;
            #[path = "../../../../↩️discard-response/🦀️.rs"]
            pub mod discard_response;
            #[path = "../../../../🚚️move-question/🦀️.rs"]
            pub mod move_question;
            #[path = "../../../../↔️move-step/🦀️.rs"]
            pub mod move_step;
            #[path = "../../../../🎛️patch-question-options/🦀️.rs"]
            pub mod patch_question_options;
            #[path = "../../../../🩹️patch-questions/🦀️.rs"]
            pub mod patch_questions;
            #[path = "../../../../✏️patch-step/🦀️.rs"]
            pub mod patch_step;
            #[path = "../../../../🩺️patch-vector-field/🦀️.rs"]
            pub mod patch_vector_field;
            #[path = "../../../../🗑️remove-question/🦀️.rs"]
            pub mod remove_question;
            #[path = "../../../../⭕️remove-question-option/🦀️.rs"]
            pub mod remove_question_option;
            #[path = "../../../../➖️remove-step/🦀️.rs"]
            pub mod remove_step;
            #[path = "../../../../🚫️remove-vector-field/🦀️.rs"]
            pub mod remove_vector_field;
            #[path = "../../../../📥️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "../../../../🧩️set-contributions/🦀️.rs"]
            pub mod set_contributions;
            #[path = "../../../../🔣️set-spec-json/🦀️.rs"]
            pub mod set_spec_json;
            #[path = "../../../../♻️update-form/🦀️.rs"]
            pub mod update_form;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod blueprint {
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_forms_forms_tests_2::editor::forms::modes::blueprint::windows::try_wizard;
                }
            }
        }
    }
}
