//! ↩️ `change-frame-action-start-ratio` inverse: restores the previous `start_ratio`.

use crate::change_frame_action_start_ratio::ChangeFrameActionStartRatio;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFrameActionStartRatio, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_frame_action_start_ratio::ChangeFrameActionStartRatio> {
    Ok(match base.frame_actions.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFrameActionStartRatio { id: payload.id.clone(), new_start_ratio: row.start_ratio })],
        None => Vec::new(),
    })
}
