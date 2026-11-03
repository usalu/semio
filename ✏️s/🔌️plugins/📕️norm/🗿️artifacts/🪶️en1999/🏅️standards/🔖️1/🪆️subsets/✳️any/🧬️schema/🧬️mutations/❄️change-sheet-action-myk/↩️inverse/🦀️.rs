//! ↩️ `change-sheet-action-myk` inverse: restores the previous `m_y_k`.

use crate::change_sheet_action_m_y_k::ChangeSheetActionMYK;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeSheetActionMYK, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_sheet_action_m_y_k::ChangeSheetActionMYK> {
    Ok(match base.cold_formed.iter().find(|row| row.id == payload.cold_formed_id).and_then(|row| row.actions.iter().find(|row| row.id == payload.action_id)) {
        Some(row) => vec![Op::from(ChangeSheetActionMYK { cold_formed_id: payload.cold_formed_id.clone(), action_id: payload.action_id.clone(), new_m_y_k: row.m_y_k })],
        None => Vec::new(),
    })
}
