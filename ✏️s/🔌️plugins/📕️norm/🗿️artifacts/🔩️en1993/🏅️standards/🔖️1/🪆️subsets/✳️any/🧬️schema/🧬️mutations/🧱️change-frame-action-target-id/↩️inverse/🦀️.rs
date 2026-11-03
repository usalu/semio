//! ↩️ `change-frame-action-target-id` inverse: restores the previous `target_id`.

use crate::change_frame_action_target_id::ChangeFrameActionTargetId;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFrameActionTargetId, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_frame_action_target_id::ChangeFrameActionTargetId> {
    Ok(match base.frame_actions.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFrameActionTargetId { id: payload.id.clone(), new_target_id: row.target_id.clone() })],
        None => Vec::new(),
    })
}
