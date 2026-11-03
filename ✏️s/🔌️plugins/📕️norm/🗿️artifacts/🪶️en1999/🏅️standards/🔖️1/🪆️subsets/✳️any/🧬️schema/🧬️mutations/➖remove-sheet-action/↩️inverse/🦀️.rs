//! ↩️ `remove-sheet-action` inverse.

use crate::remove_sheet_action::RemoveSheetAction;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::insert_sheet_action;

pub fn inverse<Op>(payload: &RemoveSheetAction, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_sheet_action::InsertSheetAction> {
    Ok((|| {
    match base.cold_formed.iter().find(|row| row.id == payload.cold_formed_id).and_then(|parent| parent.actions.iter().enumerate().find(|(_, row)| row.id == payload.id)) {
        Some((index, row)) => vec![Op::from(insert_sheet_action::InsertSheetAction { cold_formed_id: payload.cold_formed_id.clone(), index: index as u32, action: row.clone() })],
        None => Vec::new(),
    }
    })())
}
