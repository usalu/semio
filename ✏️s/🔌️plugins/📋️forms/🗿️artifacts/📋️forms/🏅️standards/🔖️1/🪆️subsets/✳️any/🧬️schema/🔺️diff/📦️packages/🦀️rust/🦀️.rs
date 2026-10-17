//! 🔺️ diff of `semio-s-artifact-forms-forms` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as framework_schema;

pub use crate::playbook::PlaybookBlock as FormQuestion;
pub use crate::playbook::PlaybookStep as FormStep;
pub use crate::schema::snapshot::FormsSnapshot;
pub use crate::schema::diff::FormsDiff;
pub use crate::playbook::PlaybookExpr as FormExpr;
pub use crate::playbook::PlaybookBlockOption as FormQuestionOption;
pub use crate::playbook::PlaybookVectorField as FormVectorField;
use semio_framework_artifact_playbook_playbook as playbook;
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_forms_forms_snapshot::{forms_children_from_steps, forms_results_child, forms_steps};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
