//! ↩️ `change-sheet-action-source` inverse: restores the previous `source`.

use crate::change_sheet_action_source::ChangeSheetActionSource;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeSheetActionSource, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_sheet_action_source::ChangeSheetActionSource> {
    Ok(match base.cold_formed.iter().find(|row| row.id == payload.cold_formed_id).and_then(|row| row.actions.iter().find(|row| row.id == payload.action_id)) {
        Some(row) => vec![Op::from(ChangeSheetActionSource { cold_formed_id: payload.cold_formed_id.clone(), action_id: payload.action_id.clone(), new_source: row.source })],
        None => Vec::new(),
    })
}
