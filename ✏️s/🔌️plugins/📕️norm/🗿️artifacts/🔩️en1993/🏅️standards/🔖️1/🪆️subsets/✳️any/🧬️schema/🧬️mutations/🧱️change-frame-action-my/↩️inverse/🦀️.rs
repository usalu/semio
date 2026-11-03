//! ↩️ `change-frame-action-my` inverse: restores the previous `my`.

use crate::change_frame_action_my::ChangeFrameActionMy;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFrameActionMy, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_frame_action_my::ChangeFrameActionMy> {
    Ok(match base.frame_actions.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFrameActionMy { id: payload.id.clone(), new_my: row.my })],
        None => Vec::new(),
    })
}
