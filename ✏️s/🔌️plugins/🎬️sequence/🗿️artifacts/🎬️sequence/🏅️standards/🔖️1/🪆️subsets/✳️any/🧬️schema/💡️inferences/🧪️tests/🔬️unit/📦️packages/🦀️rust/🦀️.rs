//! 🔬️ tests of `semio-s-artifact-sequence-sequence` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_infinite as infinite_board_port_directed_dag;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

pub use crate::snapshot::schema::SequenceSnapshot;
pub use crate::snapshot::schema::SequenceHostSnapshot;
pub use crate::schema::mutations::SequenceMutation;
pub use crate::snapshot::schema::default_host_snapshot;
pub use crate::snapshot::schema::default_snapshot;
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_sequence_sequence_schema::{SEQUENCE_DOCUMENT_SCHEMA, SequenceCamera, SequenceEdge, SequenceStep, SequenceWorkingScene, SlotRef, StepParams, genesis_sequence_child_pack, sequence_content_snapshot_from_working, sequence_derivable_scene, working_from_sequence_content_snapshot};
pub use crate::viewer::sequence::artifact_items_2::{SEQUENCE_DIALECT};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
