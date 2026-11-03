//! ↩️ `change-frame-action-end-ratio` inverse: restores the previous `end_ratio`.

use crate::change_frame_action_end_ratio::ChangeFrameActionEndRatio;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFrameActionEndRatio, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_frame_action_end_ratio::ChangeFrameActionEndRatio> {
    Ok(match base.frame_actions.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFrameActionEndRatio { id: payload.id.clone(), new_end_ratio: row.end_ratio })],
        None => Vec::new(),
    })
}
