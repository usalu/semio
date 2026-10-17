//! 📸️ snapshot of `semio-s-artifact-forms-forms` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

pub use crate::schema::snapshot::FormsSnapshot;
pub use crate::playbook::PlaybookExpr as FormExpr;
pub use crate::playbook::PlaybookBlock as FormQuestion;
pub use crate::playbook::PlaybookBlockOption as FormQuestionOption;
pub use crate::playbook::PlaybookVectorField as FormVectorField;
pub use crate::playbook::PlaybookStep as FormStep;
use semio_framework_artifact_playbook_playbook as playbook;
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use crate::standards::v1::subsets::any::io::binary::snapshot::artifact_items::{FORMS_DIALECT, FORMS_DOCUMENT_SCHEMA, FORM_BUILTIN_KINDS, FormValidationError};
pub use crate::standards::v1::subsets::any::io::binary::snapshot::artifact_items_2::{FormsResultsChild, FormsStructureChild, as_playbook_spec, block_from_semio_value, dsl_from_semio_value, expr_from_semio_value, forms_children_from_steps, forms_genesis_child_pack, forms_results_child, forms_results_from_responses, forms_scene_id, forms_snapshot_with_state, forms_steps, forms_steps_from_structure, forms_structure_from_steps, replace_forms_steps, semio_bool, semio_f64, semio_str, semio_value_from_block, semio_value_from_dsl, semio_value_from_expr, semio_value_from_step, semio_value_map_get, step_from_semio_value};

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
                    pub use semio_s_artifact_forms_forms_definition::standards::v1::subsets::any::schema::definition;
                    pub use semio_s_artifact_forms_forms_definition::standards::v1::subsets::any::schema::response;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                    #[path = "."]
                    pub mod sqlite {
                        #[path = "../../../../🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                }
            }
        }
    }
}
