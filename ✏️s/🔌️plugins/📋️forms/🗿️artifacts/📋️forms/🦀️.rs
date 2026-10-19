//! 📋️ Forms artifact — the document entity this plugin's app edits.
//!
//! Domain step/block/expr types live in the shared `playbook` kernel crate and are re-exported here under
//! forms' historical names. `FormsSnapshot` is defined in `📸️snapshot/🧬️schema` and re-exported here.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
mod art_forms_demo_tests;
extern crate semio_framework_schema as framework_schema;

pub use semio_s_artifact_forms_forms_snapshot::{FORMS_DIALECT, FORMS_DOCUMENT_SCHEMA, FORM_BUILTIN_KINDS, FormExpr, FormQuestion, FormQuestionOption, FormStep, FormValidationError, FormVectorField};

//#region 🔖️SchemaExports
pub use crate::schema::diff::FormsDiff;
pub use crate::schema::mutations::FormMutation;
pub use crate::schema::snapshot::FormsSnapshot;
//#endregion 🔖️SchemaExports

pub use semio_s_artifact_forms_forms_snapshot::{FormsResultsChild, FormsStructureChild, as_playbook_spec, block_from_semio_value, dsl_from_semio_value, expr_from_semio_value, forms_children_from_steps, forms_genesis_child_pack, forms_results_child, forms_results_from_responses, forms_scene_id, forms_snapshot_with_state, forms_steps, forms_steps_from_structure, forms_structure_from_steps, replace_forms_steps, semio_bool, semio_f64, semio_str, semio_value_from_block, semio_value_from_dsl, semio_value_from_expr, semio_value_from_step, semio_value_map_get, step_from_semio_value};

pub use semio_s_artifact_forms_forms_forms::{artifact_kind};

pub use semio_s_artifact_forms_forms_forms::{FormsApplication, artifact, definition};

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        pub use semio_s_artifact_forms_forms_forms::standards::v1::*;

        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                pub use semio_s_artifact_forms_forms_forms::standards::v1::subsets::any::*;

                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_forms_forms_schema::standards::v1::subsets::any::schema::*;

                    pub use semio_s_artifact_forms_forms_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_forms_forms_diff::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_forms_forms_mutations::standards::v1::subsets::any::schema::mutations::*;

                        pub use semio_s_artifact_forms_forms_tests_2::standards::v1::subsets::any::schema::mutations::apply;
                        #[path = "."]
                        pub mod move_block_to_step {
                            pub use semio_s_artifact_forms_forms_diff::standards::v1::subsets::any::schema::mutations::move_block_to_step::diff;
                            pub use semio_s_artifact_forms_forms_diff::standards::v1::subsets::any::schema::mutations::move_block_to_step::inverse;
                            pub use semio_s_artifact_forms_forms_diff::standards::v1::subsets::any::schema::mutations::move_block_to_step::mutation;

                        }
                        #[path = "."]
                        pub mod reorder_step {
                            pub use semio_s_artifact_forms_forms_diff::standards::v1::subsets::any::schema::mutations::reorder_step::diff;
                            pub use semio_s_artifact_forms_forms_diff::standards::v1::subsets::any::schema::mutations::reorder_step::inverse;
                            pub use semio_s_artifact_forms_forms_diff::standards::v1::subsets::any::schema::mutations::reorder_step::mutation;

                        }
                        #[path = "."]
                        pub mod create_block {
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::create_block::diff;
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::create_block::inverse;
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::create_block::mutation;

                        }
                        #[path = "."]
                        pub mod create_step {
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::create_step::diff;
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::create_step::inverse;
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::create_step::mutation;

                        }
                        #[path = "."]
                        pub mod delete_block {
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::delete_block::diff;
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::delete_block::inverse;
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::delete_block::mutation;

                        }
                        #[path = "."]
                        pub mod delete_step {
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::delete_step::diff;
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::delete_step::inverse;
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::delete_step::mutation;

                        }
                        #[path = "."]
                        pub mod change_form_title {
                            pub use semio_s_artifact_forms_forms_diff::standards::v1::subsets::any::schema::mutations::change_form_title::diff;
                            pub use semio_s_artifact_forms_forms_diff::standards::v1::subsets::any::schema::mutations::change_form_title::inverse;
                            pub use semio_s_artifact_forms_forms_diff::standards::v1::subsets::any::schema::mutations::change_form_title::mutation;

                        }
                        #[path = "."]
                        pub mod replace_block {
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::replace_block::diff;
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::replace_block::inverse;
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::replace_block::mutation;

                        }
                        #[path = "."]
                        pub mod change_block_field {
                            pub use semio_s_artifact_forms_forms_diff::standards::v1::subsets::any::schema::mutations::change_block_field::diff;
                            pub use semio_s_artifact_forms_forms_diff::standards::v1::subsets::any::schema::mutations::change_block_field::inverse;
                            pub use semio_s_artifact_forms_forms_diff::standards::v1::subsets::any::schema::mutations::change_block_field::mutation;

                        }
                        #[path = "."]
                        pub mod change_step_description {
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::change_step_description::diff;
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::change_step_description::inverse;
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::change_step_description::mutation;

                        }
                        #[path = "."]
                        pub mod rename_step {
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::rename_step::diff;
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::rename_step::inverse;
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::rename_step::mutation;

                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_forms_forms_tests_3::standards::v1::subsets::any::io::*;

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
                                pub use semio_s_artifact_forms_forms_archive_codecs::standards::v1::subsets::any::io::export::serializers::artifacts::csv;
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

// ---- Shims: keep pre-migration module paths resolving for external callers ----
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}

pub use semio_s_artifact_forms_forms_mutations::op;

pub use semio_s_artifact_forms_forms_diff::diff;
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub use semio_s_artifact_forms_forms_diff::snapshot;

pub use semio_s_artifact_forms_forms_definition::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod forms {
        pub use semio_s_artifact_forms_forms_forms::editor::forms::*;
pub use semio_s_artifact_forms_forms_definition::editor::forms::demo_session;

        pub use semio_s_artifact_forms_forms_definition::editor::forms::presence;

        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_forms_forms_config::editor::forms::config::*;
pub mod io {
    pub use semio_s_artifact_forms_forms_definition::editor::forms::config::io::*;
    pub mod binary {
        pub use semio_s_artifact_forms_forms_tests::editor::forms::config::io::binary::*;
        pub use semio_s_artifact_forms_forms_config::editor::forms::config::io::binary::mutations;
    }
    pub mod text {
        pub use semio_s_artifact_forms_forms_tests::editor::forms::config::io::text::*;
        pub use semio_s_artifact_forms_forms_config::editor::forms::config::io::text::mutations;
    }
}

            pub use semio_s_artifact_forms_forms_tests::editor::forms::config::schema;
        }

        pub use semio_s_artifact_forms_forms_tests::editor::forms::terminology;

        pub use semio_s_artifact_forms_forms_tests::editor::forms::support;

        pub use semio_s_artifact_forms_forms_schema::editor::forms::contributions;

        pub use semio_s_artifact_forms_forms_commands::editor::forms::commands;

        #[path = "."]
        pub mod modes {
            pub use semio_s_artifact_forms_forms_schema::editor::forms::modes::responses;
            pub use semio_s_artifact_forms_forms_tests::editor::forms::modes::fill;
            #[path = "."]
            pub mod blueprint {
                pub use semio_s_artifact_forms_forms_tests_2::editor::forms::modes::blueprint::*;

                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_forms_forms_tests_2::editor::forms::modes::blueprint::windows::builder;
                    pub mod try_wizard {
    pub use semio_s_artifact_forms_forms_tests_2::editor::forms::modes::blueprint::windows::try_wizard::*;
    pub mod config {
        pub use semio_s_artifact_forms_forms_tests_2::editor::forms::modes::blueprint::windows::try_wizard::config::*;
        pub use semio_s_artifact_forms_forms_tests::editor::forms::modes::blueprint::windows::try_wizard::config::schema;
    }
    pub mod transient {
        pub use semio_s_artifact_forms_forms_tests_2::editor::forms::modes::blueprint::windows::try_wizard::transient::*;
        pub use semio_s_artifact_forms_forms_tests::editor::forms::modes::blueprint::windows::try_wizard::transient::schema;
    }
}
                }
            }
        }

        #[path = "."]
        pub mod panels {
            pub use semio_s_artifact_forms_forms_tests_2::editor::forms::panels::catalogue;
            pub use semio_s_artifact_forms_forms_tests_2::editor::forms::panels::document;
            pub use semio_s_artifact_forms_forms_tests_3::editor::forms::panels::inspection;
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod forms {
        pub use semio_s_artifact_forms_forms_tests_2::viewer::forms::*;

        pub use semio_s_artifact_forms_forms_tests::viewer::forms::presence;
        pub use semio_s_artifact_forms_forms_tests::viewer::forms::config;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_forms_forms_tests_2::viewer::forms::modes::view::*;

                pub use semio_s_artifact_forms_forms_tests_2::viewer::forms::modes::view::windows;
            }
        }
    }
}

pub use semio_s_artifact_forms_forms_tests_2::{forms_child_restore_projection};

pub use semio_s_artifact_forms_forms_tests_3::host;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_0;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_1;
