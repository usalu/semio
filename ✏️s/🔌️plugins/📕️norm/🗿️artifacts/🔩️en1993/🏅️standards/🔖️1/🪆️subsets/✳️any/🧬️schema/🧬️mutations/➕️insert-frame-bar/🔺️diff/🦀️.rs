//! ➕️ `insert-frame-bar` diff — inserts the row at its position, clamped to the end of the collection.

use crate::insert_frame_bar::InsertFrameBar;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FrameBarDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &InsertFrameBar, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.frame_bars.iter().any(|existing| existing.id == payload.frame_bar.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Frame bar id {} already exists.", payload.frame_bar.id), [payload.frame_bar.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.frame_bars.len());
    protocol::MutationOutcome::new(En1993Diff { frame_bars: En1993FrameBarDelta::insertion(index, payload.frame_bar.clone()), ..Default::default() })
}
