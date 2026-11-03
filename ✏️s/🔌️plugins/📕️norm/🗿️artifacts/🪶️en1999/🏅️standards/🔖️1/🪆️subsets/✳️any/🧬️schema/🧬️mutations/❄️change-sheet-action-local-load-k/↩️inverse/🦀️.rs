//! ↩️ `change-sheet-action-local-load-k` inverse: restores the previous `local_load_k`.

use crate::change_sheet_action_local_load_k::ChangeSheetActionLocalLoadK;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeSheetActionLocalLoadK, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_sheet_action_local_load_k::ChangeSheetActionLocalLoadK> {
    Ok(match base.cold_formed.iter().find(|row| row.id == payload.cold_formed_id).and_then(|row| row.actions.iter().find(|row| row.id == payload.action_id)) {
        Some(row) => vec![Op::from(ChangeSheetActionLocalLoadK { cold_formed_id: payload.cold_formed_id.clone(), action_id: payload.action_id.clone(), new_local_load_k: row.local_load_k })],
        None => Vec::new(),
    })
}
