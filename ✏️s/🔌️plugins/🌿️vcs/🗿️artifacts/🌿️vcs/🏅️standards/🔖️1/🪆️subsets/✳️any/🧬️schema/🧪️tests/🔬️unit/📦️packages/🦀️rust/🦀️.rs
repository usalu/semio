//! 🔬️ tests of `semio-s-artifact-vcs-vcs` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::standards::v1::subsets::any::schema::snapshot::VcsSnapshot;
pub use crate::standards::v1::subsets::any::schema::mutations::VcsDemoMutation;
#[cfg(test)]
pub use crate::standards::v1::subsets::any::schema::diff::VcsDiff;
#[cfg(test)]
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub use semio_s_artifact_vcs_vcs_inferences::{VCS_DOCUMENT_SCHEMA};
pub use semio_s_artifact_vcs_vcs_editor::{VCS_DIALECT};
pub use crate::viewer::vcs::artifact_items_5::{VcsApplication};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
