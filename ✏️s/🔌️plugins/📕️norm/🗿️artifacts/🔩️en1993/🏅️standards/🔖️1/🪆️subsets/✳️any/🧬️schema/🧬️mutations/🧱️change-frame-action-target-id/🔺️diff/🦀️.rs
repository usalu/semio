//! 🔺️ `change-frame-action-target-id` diff: patches `target_id` of the addressed frame_action block row.

use crate::change_frame_action_target_id::ChangeFrameActionTargetId;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FrameActionDelta, En1993FrameActionPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeFrameActionTargetId, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.frame_actions.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "frame-action", Vec::<String>::new());
    };
    if row.target_id == payload.new_target_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The frame_action block already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { frame_actions: En1993FrameActionDelta::modification(payload.id.clone(), En1993FrameActionPatch { target_id: Some(payload.new_target_id.clone()), ..Default::default() }), ..Default::default() })
}
