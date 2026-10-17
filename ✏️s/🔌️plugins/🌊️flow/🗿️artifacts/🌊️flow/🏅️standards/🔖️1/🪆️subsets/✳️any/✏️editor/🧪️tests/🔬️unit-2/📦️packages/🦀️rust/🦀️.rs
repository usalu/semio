//! 🔬️ unit-tests-2 of `semio-s-artifact-flow-flow` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[cfg(test)]
pub use crate::standards::v1::subsets::any::schema::snapshot::FlowSnapshot;
#[cfg(test)]
pub use crate::standards::v1::subsets::any::schema::mutations::FlowMutation;
#[cfg(test)]
use semio_framework_artifact_playbook_playbook as playbook;
#[cfg(test)]
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_flow_flow_schema::{FLOW_DOCUMENT_SCHEMA, FlowWorkingScene, flow_content_child_handle_and_cache, flow_content_snapshot_from_working, working_from_flow_content_snapshot};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
