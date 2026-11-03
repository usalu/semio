//! 🔺️ `change-frame-node-z` diff: patches `z` of the addressed frame node row.

use crate::change_frame_node_z::ChangeFrameNodeZ;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FrameNodeDelta, En1993FrameNodePatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeFrameNodeZ, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_z.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.frame_nodes.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "frame-node", Vec::<String>::new());
    };
    if row.z == payload.new_z {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The frame node already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { frame_nodes: En1993FrameNodeDelta::modification(payload.id.clone(), En1993FrameNodePatch { z: Some(payload.new_z), ..Default::default() }), ..Default::default() })
}
