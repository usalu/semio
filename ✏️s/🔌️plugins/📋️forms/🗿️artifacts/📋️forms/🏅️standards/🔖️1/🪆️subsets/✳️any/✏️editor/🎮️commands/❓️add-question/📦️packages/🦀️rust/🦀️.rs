//! ❓️ commands of `semio-s-artifact-forms-forms` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;

pub use crate::schema::snapshot::FormsSnapshot;
pub use crate::playbook::PlaybookStep as FormStep;
pub use crate::playbook::PlaybookVectorField as FormVectorField;
pub use crate::playbook::PlaybookBlockOption as FormQuestionOption;
use semio_framework_artifact_playbook_playbook as playbook;
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_forms_forms_snapshot::{forms_steps};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
