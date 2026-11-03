//! ↩️ `change-sheet-thickness` inverse: restores the previous `thickness`.

use crate::change_sheet_thickness::ChangeSheetThickness;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeSheetThickness, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_sheet_thickness::ChangeSheetThickness> {
    Ok(match base.cold_formed.iter().find(|row| row.id == payload.cold_formed_id) {
        Some(row) => vec![Op::from(ChangeSheetThickness { cold_formed_id: payload.cold_formed_id.clone(), new_thickness: row.thickness })],
        None => Vec::new(),
    })
}
