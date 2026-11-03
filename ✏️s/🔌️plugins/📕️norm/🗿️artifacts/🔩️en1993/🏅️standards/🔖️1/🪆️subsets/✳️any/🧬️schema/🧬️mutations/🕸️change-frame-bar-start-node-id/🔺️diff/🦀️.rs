//! 🔺️ `change-frame-bar-start-node-id` diff: patches `start_node_id` of the addressed frame bar row.

use crate::change_frame_bar_start_node_id::ChangeFrameBarStartNodeId;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FrameBarDelta, En1993FrameBarPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeFrameBarStartNodeId, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.frame_bars.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "frame-bar", Vec::<String>::new());
    };
    if row.start_node_id == payload.new_start_node_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The frame bar already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { frame_bars: En1993FrameBarDelta::modification(payload.id.clone(), En1993FrameBarPatch { start_node_id: Some(payload.new_start_node_id.clone()), ..Default::default() }), ..Default::default() })
}
