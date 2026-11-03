//! ➖️ `remove-frame-node` diff — removes the row at the index.

use crate::remove_frame_node::RemoveFrameNode;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FrameNodeDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemoveFrameNode, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.frame_nodes.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("frame-node index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { frame_nodes: En1993FrameNodeDelta::removal(&base.frame_nodes, payload.index), ..Default::default() })
}
