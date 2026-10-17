//! ✏️ editor of `semio-s-artifact-vcs-vcs` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::standards::v1::subsets::any::schema::snapshot::VcsSnapshot;
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub mod op {
    pub use crate::standards::v1::subsets::any::schema::mutations::VcsDemoMutation;
}
pub use semio_s_artifact_vcs_vcs_inferences::{VCS_DOCUMENT_SCHEMA};
pub use crate::editor::vcs::artifact_items_2::{VCS_DIALECT};
pub use crate::editor::vcs::artifact_items_3::{artifact_kind};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
