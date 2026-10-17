//! 🧬️ mutations of `semio-s-artifact-forms-forms` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::playbook::PlaybookBlock as FormQuestion;
pub use crate::schema::diff::FormsDiff;
pub use crate::schema::snapshot::FormsSnapshot;
pub use crate::schema::mutations::FormMutation;
pub use crate::playbook::PlaybookStep as FormStep;
pub use crate::playbook::PlaybookBlockOption as FormQuestionOption;
use semio_framework_artifact_playbook_playbook as playbook;
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_forms_forms_snapshot::{replace_forms_steps};

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
                    pub use semio_s_artifact_forms_forms_diff::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
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
                        pub mod replace_block {
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::replace_block::mutation;
                        }
                        #[path = "."]
                        pub mod change_block_field {
                            pub use semio_s_artifact_forms_forms_diff::standards::v1::subsets::any::schema::mutations::change_block_field::mutation;
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
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod snapshot {
                            #[path = "."]
                            pub mod response {
                                pub use semio_s_artifact_forms_forms_definition::standards::v1::subsets::any::io::text::snapshot::response::export;
                            }
                        }
                        #[path = "../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                }
            }
        }
    }
}
pub mod op {
    pub use crate::standards::v1::subsets::any::schema::mutations::{inverse_form_mutation, FormMutation};

}
