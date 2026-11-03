//! ↩️ `change-sheet-welded` inverse: restores the previous `welded`.

use crate::change_sheet_welded::ChangeSheetWelded;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeSheetWelded, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_sheet_welded::ChangeSheetWelded> {
    Ok(match base.cold_formed.iter().find(|row| row.id == payload.cold_formed_id) {
        Some(row) => vec![Op::from(ChangeSheetWelded { cold_formed_id: payload.cold_formed_id.clone(), new_welded: row.welded })],
        None => Vec::new(),
    })
}
