//! 🔺️ `change-frame-bar-release-start` diff: patches `release_start` of the addressed frame bar row.

use crate::change_frame_bar_release_start::ChangeFrameBarReleaseStart;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FrameBarDelta, En1993FrameBarPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeFrameBarReleaseStart, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.frame_bars.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "frame-bar", Vec::<String>::new());
    };
    if row.release_start == payload.new_release_start {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The frame bar already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { frame_bars: En1993FrameBarDelta::modification(payload.id.clone(), En1993FrameBarPatch { release_start: Some(payload.new_release_start), ..Default::default() }), ..Default::default() })
}
