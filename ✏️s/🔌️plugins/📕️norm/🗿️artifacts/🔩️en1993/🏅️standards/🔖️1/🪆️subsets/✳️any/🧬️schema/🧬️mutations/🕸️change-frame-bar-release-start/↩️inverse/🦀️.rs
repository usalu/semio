//! ↩️ `change-frame-bar-release-start` inverse: restores the previous `release_start`.

use crate::change_frame_bar_release_start::ChangeFrameBarReleaseStart;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFrameBarReleaseStart, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_frame_bar_release_start::ChangeFrameBarReleaseStart> {
    Ok(match base.frame_bars.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFrameBarReleaseStart { id: payload.id.clone(), new_release_start: row.release_start })],
        None => Vec::new(),
    })
}
