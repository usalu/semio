//! 🔺️ `change-frame-action-load-case-id` diff: patches `load_case_id` of the addressed frame_action block row.

use crate::change_frame_action_load_case_id::ChangeFrameActionLoadCaseId;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FrameActionDelta, En1993FrameActionPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeFrameActionLoadCaseId, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.frame_actions.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "frame-action", Vec::<String>::new());
    };
    if row.load_case_id == payload.new_load_case_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The frame_action block already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { frame_actions: En1993FrameActionDelta::modification(payload.id.clone(), En1993FrameActionPatch { load_case_id: Some(payload.new_load_case_id.clone()), ..Default::default() }), ..Default::default() })
}
