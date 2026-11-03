//! ➖️ `remove-frame-action` diff — removes the row at the index.

use crate::remove_frame_action::RemoveFrameAction;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FrameActionDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemoveFrameAction, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.frame_actions.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("frame-action index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { frame_actions: En1993FrameActionDelta::removal(&base.frame_actions, payload.index), ..Default::default() })
}
