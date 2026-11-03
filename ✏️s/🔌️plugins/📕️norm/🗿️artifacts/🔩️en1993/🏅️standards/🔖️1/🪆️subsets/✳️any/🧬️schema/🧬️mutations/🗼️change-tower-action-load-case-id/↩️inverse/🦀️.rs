//! ↩️ `change-tower-action-load-case-id` inverse: restores the previous `load_case_id`.

use crate::change_tower_action_load_case_id::ChangeTowerActionLoadCaseId;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerActionLoadCaseId, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_action_load_case_id::ChangeTowerActionLoadCaseId> {
    Ok(match base.tower_legs.iter().find(|row| row.id == payload.id).and_then(|row| row.actions.iter().find(|row| row.id == payload.element_id)) {
        Some(row) => vec![Op::from(ChangeTowerActionLoadCaseId { id: payload.id.clone(), element_id: payload.element_id.clone(), new_load_case_id: row.load_case_id.clone() })],
        None => Vec::new(),
    })
}
