//! ↩️ `change-sheet-width` inverse: restores the previous `width`.

use crate::change_sheet_width::ChangeSheetWidth;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeSheetWidth, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_sheet_width::ChangeSheetWidth> {
    Ok(match base.cold_formed.iter().find(|row| row.id == payload.cold_formed_id) {
        Some(row) => vec![Op::from(ChangeSheetWidth { cold_formed_id: payload.cold_formed_id.clone(), new_width: row.width })],
        None => Vec::new(),
    })
}
