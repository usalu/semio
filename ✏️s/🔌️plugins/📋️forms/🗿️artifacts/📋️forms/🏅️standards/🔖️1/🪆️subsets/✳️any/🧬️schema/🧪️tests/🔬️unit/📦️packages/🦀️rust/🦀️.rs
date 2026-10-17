//! 🔬️ tests-3 of `semio-s-artifact-forms-forms` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::schema::mutations::FormMutation;
pub use crate::playbook::PlaybookBlock as FormQuestion;
pub use crate::playbook::PlaybookStep as FormStep;
pub use crate::schema::snapshot::FormsSnapshot;
pub use crate::playbook::PlaybookExpr as FormExpr;
use semio_framework_artifact_playbook_playbook as playbook;
#[cfg(test)]
pub use crate::playbook::PlaybookBlockOption as FormQuestionOption;
#[cfg(test)]
pub use crate::schema::diff::FormsDiff;
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_forms_forms_snapshot::{FORMS_DIALECT, FORMS_DOCUMENT_SCHEMA, forms_results_child, forms_snapshot_with_state, forms_steps, replace_forms_steps};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
