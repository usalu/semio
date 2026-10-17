//! 💡️ inferences of `semio-s-artifact-sequence-sequence` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_infinite as infinite_board_port_directed_dag;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

pub use crate::snapshot::schema::SequenceSnapshot;
pub use crate::snapshot::schema::default_host_snapshot;
pub use crate::snapshot::schema::default_snapshot;
pub use crate::snapshot::schema::SequenceHostSnapshot;
#[cfg(test)]
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_sequence_sequence_schema::{SequenceContentChild, SequenceEdge, SequenceStep, SequenceWorkingScene, SlotRef, StepParams, sequence_content_snapshot_from_working, sequence_derivable_scene};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
