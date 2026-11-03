//! 🔺️ `change-frame-action-fz` diff: patches `fz` of the addressed frame_action block row.

use crate::change_frame_action_fz::ChangeFrameActionFz;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FrameActionDelta, En1993FrameActionPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeFrameActionFz, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_fz.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.frame_actions.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "frame-action", Vec::<String>::new());
    };
    if row.fz == payload.new_fz {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The frame_action block already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { frame_actions: En1993FrameActionDelta::modification(payload.id.clone(), En1993FrameActionPatch { fz: Some(payload.new_fz), ..Default::default() }), ..Default::default() })
}
