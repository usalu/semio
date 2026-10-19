//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "../../../../../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "../../../../🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_forms_forms_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_forms_forms_definition::standards::v1::subsets::any::schema::definition;
                    pub use semio_s_artifact_forms_forms_definition::standards::v1::subsets::any::schema::response;
                    pub use semio_s_artifact_forms_forms_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::inferences;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_forms_forms_mutations::standards::v1::subsets::any::schema::mutations::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_forms_forms_tests_3::standards::v1::subsets::any::io::*;
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
        #[path = "../../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod questions {
            pub use semio_s_artifact_forms_forms_questions::editor::forms::questions::*;
            #[path = "."]
            pub mod placement {
                pub use semio_s_artifact_forms_forms_tests_3::editor::forms::questions::placement::*;
                #[cfg(test)]
                #[path = "../../../❓️questions/📍️placement/🧪️tests/🦀️.rs"]
                pub mod tests;
            }
            #[cfg(test)]
            #[path = "../../../❓️questions/🧪️tests/🔬️patches/🦀️.rs"]
            pub mod patch_tests;
        }
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_forms_forms_config::editor::forms::config::*;
            pub use semio_s_artifact_forms_forms_tests::editor::forms::config::schema;
        }
        pub use semio_s_artifact_forms_forms_tests::editor::forms::terminology;
        pub use semio_s_artifact_forms_forms_tests::editor::forms::support;
        pub use semio_s_artifact_forms_forms_schema::editor::forms::contributions;
        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_forms_forms_commands::editor::forms::commands::*;
            #[path = "."]
            pub mod set_contributions {
                pub use semio_s_artifact_forms_forms_commands::editor::forms::commands::set_contributions::*;
                #[cfg(test)]
                #[path = "../../../🎮️commands/🧩️set-contributions/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod export_snapshot {
                pub use semio_s_artifact_forms_forms_commands::editor::forms::commands::export_snapshot::*;
                #[cfg(test)]
                #[path = "../../../🎮️commands/📤️export-snapshot/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod add_step {
                pub use semio_s_artifact_forms_forms_commands::editor::forms::commands::add_step::*;
                #[cfg(test)]
                #[path = "../../../🎮️commands/📃️add-step/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod add_question_option {
                pub use semio_s_artifact_forms_forms_commands::editor::forms::commands::add_question_option::*;
                #[cfg(test)]
                #[path = "../../../🎮️commands/🔘️add-question-option/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod add_question {
                pub use semio_s_artifact_forms_forms_commands::editor::forms::commands::add_question::*;
                #[cfg(test)]
                #[path = "../../../🎮️commands/❓️add-question/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod patch_vector_field {
                pub use semio_s_artifact_forms_forms_commands::editor::forms::commands::patch_vector_field::*;
                #[cfg(test)]
                #[path = "../../../🎮️commands/🩺️patch-vector-field/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod set_spec_json {
                pub use semio_s_artifact_forms_forms_commands::editor::forms::commands::set_spec_json::*;
                #[cfg(test)]
                #[path = "../../../🎮️commands/🔣️set-spec-json/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod submit {
                #[cfg(test)]
                #[path = "../../../🎮️commands/✅️submit/🧪️tests/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod set_try_value {
                #[cfg(test)]
                #[path = "../../../🎮️commands/🎯️set-try-value/🧪️tests/🔬️chunk-value-vectors/🦀️.rs"]
                pub mod chunk_value_vectors;
            }
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod responses {
                pub use semio_s_artifact_forms_forms_schema::editor::forms::modes::responses::*;
                pub use semio_s_artifact_forms_forms_diff::editor::forms::modes::responses::results;
            }
            pub use semio_s_artifact_forms_forms_tests::editor::forms::modes::fill;
            #[path = "."]
            pub mod blueprint {
                pub use semio_s_artifact_forms_forms_tests_2::editor::forms::modes::blueprint::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_forms_forms_tests_2::editor::forms::modes::blueprint::windows::*;
                    #[path = "."]
                    pub mod builder {
                        pub use semio_s_artifact_forms_forms_tests_2::editor::forms::modes::blueprint::windows::builder::*;
                        #[cfg(test)]
                        #[path = "../../../🎭️modes/📝️blueprint/🪟️windows/🧱️builder/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod try_wizard {
                        pub use semio_s_artifact_forms_forms_tests_2::editor::forms::modes::blueprint::windows::try_wizard::*;
                        #[cfg(test)]
                        #[path = "../../../🎭️modes/📝️blueprint/🪟️windows/▶️try/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
        #[path = "."]
        pub mod panels {
            #[path = "."]
            pub mod catalogue {
                pub use semio_s_artifact_forms_forms_tests_2::editor::forms::panels::catalogue::*;
                #[cfg(test)]
                #[path = "../../../📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod document {
                pub use semio_s_artifact_forms_forms_tests_2::editor::forms::panels::document::*;
                #[cfg(test)]
                #[path = "../../../📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod inspection {
                pub use semio_s_artifact_forms_forms_tests_3::editor::forms::panels::inspection::*;
                #[cfg(test)]
                #[path = "../../../📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
                #[cfg(test)]
                #[path = "../../../📌️panels/🔍️inspection/🧪️tests/🔬️authoring/🦀️.rs"]
                pub mod authoring_tests;
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod forms {
        pub use semio_s_artifact_forms_forms_tests_2::viewer::forms::*;
    }
}
