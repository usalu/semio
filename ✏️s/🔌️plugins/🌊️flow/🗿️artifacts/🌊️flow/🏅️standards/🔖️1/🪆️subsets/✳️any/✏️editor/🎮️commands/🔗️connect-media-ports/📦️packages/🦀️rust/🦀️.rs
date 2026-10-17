//! 🔗️ editor-leaves of `semio-s-artifact-flow-flow` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_value_derive as value_derive;

pub use crate::standards::v1::subsets::any::schema::mutations::FlowMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::FlowSnapshot;
use semio_framework_artifact_playbook_playbook as playbook;
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_flow_flow_schema::{flow_composed_snapshot, working_from_flow_content_snapshot};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
