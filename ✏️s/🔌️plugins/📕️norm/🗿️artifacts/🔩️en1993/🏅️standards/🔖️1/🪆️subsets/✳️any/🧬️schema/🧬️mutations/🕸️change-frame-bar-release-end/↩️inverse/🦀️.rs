//! ↩️ `change-frame-bar-release-end` inverse: restores the previous `release_end`.

use crate::change_frame_bar_release_end::ChangeFrameBarReleaseEnd;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFrameBarReleaseEnd, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_frame_bar_release_end::ChangeFrameBarReleaseEnd> {
    Ok(match base.frame_bars.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFrameBarReleaseEnd { id: payload.id.clone(), new_release_end: row.release_end })],
        None => Vec::new(),
    })
}
