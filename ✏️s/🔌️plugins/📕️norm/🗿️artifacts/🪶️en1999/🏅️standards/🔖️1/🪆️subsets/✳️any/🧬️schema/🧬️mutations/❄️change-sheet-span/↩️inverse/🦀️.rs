//! ↩️ `change-sheet-span` inverse: restores the previous `span`.

use crate::change_sheet_span::ChangeSheetSpan;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeSheetSpan, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_sheet_span::ChangeSheetSpan> {
    Ok(match base.cold_formed.iter().find(|row| row.id == payload.cold_formed_id) {
        Some(row) => vec![Op::from(ChangeSheetSpan { cold_formed_id: payload.cold_formed_id.clone(), new_span: row.span })],
        None => Vec::new(),
    })
}
