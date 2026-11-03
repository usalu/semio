//! ↩️ `change-frame-action-load-case-id` inverse: restores the previous `load_case_id`.

use crate::change_frame_action_load_case_id::ChangeFrameActionLoadCaseId;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFrameActionLoadCaseId, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_frame_action_load_case_id::ChangeFrameActionLoadCaseId> {
    Ok(match base.frame_actions.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFrameActionLoadCaseId { id: payload.id.clone(), new_load_case_id: row.load_case_id.clone() })],
        None => Vec::new(),
    })
}
