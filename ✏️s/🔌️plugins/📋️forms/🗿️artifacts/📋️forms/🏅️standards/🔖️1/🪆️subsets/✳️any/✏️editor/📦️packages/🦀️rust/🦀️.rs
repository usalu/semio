//! ✏️ forms of `semio-s-artifact-forms-forms` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::playbook::PlaybookBlock as FormQuestion;
pub use crate::schema::snapshot::FormsSnapshot;
pub use crate::playbook::PLAYBOOK_BUILTIN_KINDS as FORM_BUILTIN_KINDS;
use semio_framework_artifact_playbook_playbook as playbook;
#[cfg(test)]
pub use crate::schema::mutations::FormMutation;
#[cfg(test)]
pub use crate::playbook::PlaybookStep as FormStep;
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_forms_forms_snapshot::{FORMS_DIALECT, FORMS_DOCUMENT_SCHEMA, as_playbook_spec, forms_genesis_child_pack, forms_snapshot_with_state, forms_steps, replace_forms_steps};
pub use semio_s_artifact_forms_forms_tests_2::{forms_child_restore_projection};
pub use crate::standards::v1::artifact_items_3::{artifact_kind};
pub use crate::standards::v1::artifact_items_4::{FormsApplication, artifact, definition};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
