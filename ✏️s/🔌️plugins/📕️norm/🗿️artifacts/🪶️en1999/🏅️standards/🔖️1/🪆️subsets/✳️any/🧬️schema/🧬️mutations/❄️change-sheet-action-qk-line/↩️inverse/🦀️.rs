//! ↩️ `change-sheet-action-qk-line` inverse: restores the previous `q_k_line`.

use crate::change_sheet_action_q_k_line::ChangeSheetActionQKLine;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeSheetActionQKLine, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_sheet_action_q_k_line::ChangeSheetActionQKLine> {
    Ok(match base.cold_formed.iter().find(|row| row.id == payload.cold_formed_id).and_then(|row| row.actions.iter().find(|row| row.id == payload.action_id)) {
        Some(row) => vec![Op::from(ChangeSheetActionQKLine { cold_formed_id: payload.cold_formed_id.clone(), action_id: payload.action_id.clone(), new_q_k_line: row.q_k_line })],
        None => Vec::new(),
    })
}
