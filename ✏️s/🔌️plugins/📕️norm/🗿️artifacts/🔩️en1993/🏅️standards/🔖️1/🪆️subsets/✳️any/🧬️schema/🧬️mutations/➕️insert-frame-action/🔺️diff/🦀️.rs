//! ➕️ `insert-frame-action` diff — inserts the row at its position, clamped to the end of the collection.

use crate::insert_frame_action::InsertFrameAction;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FrameActionDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &InsertFrameAction, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.frame_actions.iter().any(|existing| existing.id == payload.frame_action.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Frame action id {} already exists.", payload.frame_action.id), [payload.frame_action.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.frame_actions.len());
    protocol::MutationOutcome::new(En1993Diff { frame_actions: En1993FrameActionDelta::insertion(index, payload.frame_action.clone()), ..Default::default() })
}
