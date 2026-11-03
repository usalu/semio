//! ↩️ `change-plated-action-load-case-id` inverse: restores the previous `load_case_id`.

use crate::change_plated_action_load_case_id::ChangePlatedActionLoadCaseId;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedActionLoadCaseId, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_action_load_case_id::ChangePlatedActionLoadCaseId> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id).and_then(|row| row.actions.iter().find(|row| row.id == payload.element_id)) {
        Some(row) => vec![Op::from(ChangePlatedActionLoadCaseId { id: payload.id.clone(), element_id: payload.element_id.clone(), new_load_case_id: row.load_case_id.clone() })],
        None => Vec::new(),
    })
}
