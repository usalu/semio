//! ↩️ `change-action-load-case-id` inverse: restores the previous `load_case_id`.

use crate::change_action_load_case_id::ChangeActionLoadCaseId;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeActionLoadCaseId, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_action_load_case_id::ChangeActionLoadCaseId> {
    Ok(match base.member_actions.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeActionLoadCaseId { id: payload.id.clone(), new_load_case_id: row.load_case_id.clone() })],
        None => Vec::new(),
    })
}
