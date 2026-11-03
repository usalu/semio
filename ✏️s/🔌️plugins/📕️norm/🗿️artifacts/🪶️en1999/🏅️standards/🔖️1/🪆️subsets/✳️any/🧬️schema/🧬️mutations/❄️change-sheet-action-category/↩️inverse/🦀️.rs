//! ↩️ `change-sheet-action-category` inverse: restores the previous `category`.

use crate::change_sheet_action_category::ChangeSheetActionCategory;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeSheetActionCategory, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_sheet_action_category::ChangeSheetActionCategory> {
    Ok(match base.cold_formed.iter().find(|row| row.id == payload.cold_formed_id).and_then(|row| row.actions.iter().find(|row| row.id == payload.action_id)) {
        Some(row) => vec![Op::from(ChangeSheetActionCategory { cold_formed_id: payload.cold_formed_id.clone(), action_id: payload.action_id.clone(), new_category: row.category })],
        None => Vec::new(),
    })
}
