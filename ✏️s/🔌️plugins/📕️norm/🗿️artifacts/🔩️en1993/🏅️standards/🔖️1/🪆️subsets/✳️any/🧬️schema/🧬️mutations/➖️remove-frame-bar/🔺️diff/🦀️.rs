//! ➖️ `remove-frame-bar` diff — removes the row at the index.

use crate::remove_frame_bar::RemoveFrameBar;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FrameBarDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemoveFrameBar, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.frame_bars.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("frame-bar index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { frame_bars: En1993FrameBarDelta::removal(&base.frame_bars, payload.index), ..Default::default() })
}
