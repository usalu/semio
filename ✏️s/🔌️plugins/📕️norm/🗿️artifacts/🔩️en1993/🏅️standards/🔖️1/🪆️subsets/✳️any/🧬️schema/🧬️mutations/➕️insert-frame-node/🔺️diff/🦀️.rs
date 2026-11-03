//! ➕️ `insert-frame-node` diff — inserts the row at its position, clamped to the end of the collection.

use crate::insert_frame_node::InsertFrameNode;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FrameNodeDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &InsertFrameNode, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.frame_nodes.iter().any(|existing| existing.id == payload.frame_node.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Frame node id {} already exists.", payload.frame_node.id), [payload.frame_node.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.frame_nodes.len());
    protocol::MutationOutcome::new(En1993Diff { frame_nodes: En1993FrameNodeDelta::insertion(index, payload.frame_node.clone()), ..Default::default() })
}
