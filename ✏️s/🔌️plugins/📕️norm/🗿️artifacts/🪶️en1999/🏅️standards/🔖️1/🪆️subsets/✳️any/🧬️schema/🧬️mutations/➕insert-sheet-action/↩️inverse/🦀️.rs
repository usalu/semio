//! ↩️ `insert-sheet-action` inverse.

use crate::insert_sheet_action::InsertSheetAction;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::remove_sheet_action;

pub fn inverse<Op>(payload: &InsertSheetAction, _base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_sheet_action::RemoveSheetAction> {
    Ok((|| {
    vec![Op::from(remove_sheet_action::RemoveSheetAction { cold_formed_id: payload.cold_formed_id.clone(), id: payload.action.id.clone() })]
    })())
}
