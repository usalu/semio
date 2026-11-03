//! 🔺️ `change-frame-action-kind` diff: patches `kind` of the addressed frame_action row.

use crate::change_frame_action_kind::ChangeFrameActionKind;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FrameActionDelta, En1993FrameActionPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeFrameActionKind, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.frame_actions.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "frame_action", Vec::<String>::new());
    };
    if row.kind == payload.new_kind {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The frame_action already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { frame_actions: En1993FrameActionDelta::modification(payload.id.clone(), En1993FrameActionPatch { kind: Some(payload.new_kind), ..Default::default() }), ..Default::default() })
}
