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
                    #[cfg(test)]
                    #[path = "../../../🦀️.rs"]
                    pub mod tests;
                    pub use semio_s_artifact_forms_forms_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_forms_forms_diff::standards::v1::subsets::any::schema::diff;
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
                        #[cfg(test)]
                        #[path = "../../../../../🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                        pub use semio_s_artifact_forms_forms_tests_2::standards::v1::subsets::any::schema::mutations::apply;
                        #[path = "."]
                        pub mod move_block_to_step {
                            pub use semio_s_artifact_forms_forms_diff::standards::v1::subsets::any::schema::mutations::move_block_to_step::mutation;
                            #[cfg(test)]
                            #[path = "../../../../../🧬️mutations/📦move-block-to-step/🧪️tests/🧪️no/🦀️.rs"]
                            pub mod tests_no_ops_when_the_block_stays_at_its_index_in_its_own_step;
                        }
                        #[path = "."]
                        pub mod reorder_step {
                            pub use semio_s_artifact_forms_forms_diff::standards::v1::subsets::any::schema::mutations::reorder_step::mutation;
                            #[cfg(test)]
                            #[path = "../../../../../🧬️mutations/🔀reorder-step/🧪️tests/🧪️no/🦀️.rs"]
                            pub mod tests_no_ops_when_the_step_already_sits_at_that_index;
                        }
                        #[path = "."]
                        pub mod create_block {
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::create_block::mutation;
                            #[cfg(test)]
                            #[path = "../../../../../🧬️mutations/➕create-block/🧪️tests/🧪️rejects/🦀️.rs"]
                            pub mod tests_rejects_a_block_for_a_step_that_does_not_exist;
                        }
                        #[path = "."]
                        pub mod create_step {
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::create_step::mutation;
                            #[cfg(test)]
                            #[path = "../../../../../🧬️mutations/🌱create-step/🧪️tests/🧪️rejects/🦀️.rs"]
                            pub mod tests_rejects_a_duplicate_step_id;
                        }
                        #[path = "."]
                        pub mod delete_block {
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::delete_block::mutation;
                            #[cfg(test)]
                            #[path = "../../../../../🧬️mutations/➖delete-block/🧪️tests/🧪️rejects/🦀️.rs"]
                            pub mod tests_rejects_deleting_a_block_missing_from_an_existing_step;
                        }
                        #[path = "."]
                        pub mod delete_step {
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::delete_step::mutation;
                            #[cfg(test)]
                            #[path = "../../../../../🧬️mutations/🗑️delete-step/🧪️tests/🧪️rejects/🦀️.rs"]
                            pub mod tests_rejects_deleting_a_step_the_scene_does_not_hold;
                        }
                        #[path = "."]
                        pub mod change_form_title {
                            pub use semio_s_artifact_forms_forms_diff::standards::v1::subsets::any::schema::mutations::change_form_title::mutation;
                            #[cfg(test)]
                            #[path = "../../../../../🧬️mutations/🏷️change-form-title/🧪️tests/🧪️titles/🦀️.rs"]
                            pub mod tests_titles_an_untitled_survey;
                        }
                        #[path = "."]
                        pub mod replace_block {
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::replace_block::mutation;
                            #[cfg(test)]
                            #[path = "../../../../../🧬️mutations/🔁replace-block/🧪️tests/🧪️no/🦀️.rs"]
                            pub mod tests_no_ops_when_the_replacement_block_is_identical;
                        }
                        #[path = "."]
                        pub mod change_block_field {
                            pub use semio_s_artifact_forms_forms_diff::standards::v1::subsets::any::schema::mutations::change_block_field::mutation;
                            #[cfg(test)]
                            #[path = "../../../../../🧬️mutations/🎛️change-block-field/🧪️tests/🧪️rejects/🦀️.rs"]
                            pub mod tests_rejects_a_field_of_a_question_the_scene_does_not_hold;
                            #[cfg(test)]
                            #[path = "../../../../../🧬️mutations/🎛️change-block-field/🧪️tests/🧪️refuses/🦀️.rs"]
                            pub mod tests_refuses_a_minimum_above_the_maximum;
                            #[cfg(test)]
                            #[path = "../../../../../🧬️mutations/🎛️change-block-field/🧪️tests/🧪️no/🦀️.rs"]
                            pub mod tests_no_ops_when_the_field_already_holds_the_value;
                        }
                        #[path = "."]
                        pub mod change_step_description {
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::change_step_description::mutation;
                            #[cfg(test)]
                            #[path = "../../../../../🧬️mutations/📝change-step-description/🧪️tests/🧪️no/🦀️.rs"]
                            pub mod tests_no_ops_when_clearing_an_already_absent_description;
                        }
                        #[path = "."]
                        pub mod rename_step {
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::rename_step::mutation;
                            #[cfg(test)]
                            #[path = "../../../../../🧬️mutations/✏️rename-step/🧪️tests/🧪️no/🦀️.rs"]
                            pub mod tests_no_ops_when_the_step_already_carries_that_title;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_forms_forms_schema::standards::v1::subsets::any::io::binary;
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_forms_forms_tests_2::standards::v1::subsets::any::io::text::*;
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_forms_forms_snapshot::standards::v1::subsets::any::io::text::snapshot::*;
                            #[path = "."]
                            pub mod response {
                                pub use semio_s_artifact_forms_forms_definition::standards::v1::subsets::any::io::text::snapshot::response::export;
                            }
                        }
                        #[path = "."]
                        pub mod mutations {
                            pub use semio_s_artifact_forms_forms_mutations::standards::v1::subsets::any::io::text::mutations::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🚪️io/📝️text/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                    pub use semio_s_artifact_forms_forms_diff::standards::v1::subsets::any::io::sqlite;
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_forms_forms_archive_codecs::standards::v1::subsets::any::io::import::deserializers::artifacts::zip;
                                pub use semio_s_artifact_forms_forms_archive_codecs::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_forms_forms_archive_codecs::standards::v1::subsets::any::io::export::serializers::artifacts::zip;
                                #[path = "."]
                                pub mod csv {
                                    pub use semio_s_artifact_forms_forms_archive_codecs::standards::v1::subsets::any::io::export::serializers::artifacts::csv::*;
                                    #[path = "."]
                                    pub mod v_rfc4180 {
                                        pub use semio_s_artifact_forms_forms_archive_codecs::standards::v1::subsets::any::io::export::serializers::artifacts::csv::v_rfc4180::*;
                                        #[path = "."]
                                        pub mod any {
                                            pub use semio_s_artifact_forms_forms_archive_codecs::standards::v1::subsets::any::io::export::serializers::artifacts::csv::v_rfc4180::any::*;
                                            #[cfg(test)]
                                            #[path = "../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🧪️tests/🔬️unit/🦀️.rs"]
                                            pub mod tests;
                                        }
                                    }
                                }
                                pub use semio_s_artifact_forms_forms_archive_codecs::standards::v1::subsets::any::io::export::serializers::artifacts::xlsx;
                                pub use semio_s_artifact_forms_forms_archive_codecs::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                            }
                        }
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_forms_forms_mutations::op;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod forms {
        #[path = "."]
        pub mod questions {
            #[path = "../../../../../../✏️editor/❓️questions/📍️placement/🦀️.rs"]
            pub mod placement;
            pub use semio_s_artifact_forms_forms_diff::editor::forms::questions::extensions;
        }
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_forms_forms_config::editor::forms::config::*;
        }
        pub use semio_s_artifact_forms_forms_tests::editor::forms::terminology;
        pub use semio_s_artifact_forms_forms_tests::editor::forms::support;
        pub use semio_s_artifact_forms_forms_schema::editor::forms::contributions;
        #[path = "."]
        pub mod panels {
            #[path = "../../../../../../✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
        }
    }
}
#[path = "."]
pub mod host {
#[path = "../../../../../../../../../../🔨️modules/🏠️host/🧰️owned/🦀️.rs"]
pub mod owned;
}
