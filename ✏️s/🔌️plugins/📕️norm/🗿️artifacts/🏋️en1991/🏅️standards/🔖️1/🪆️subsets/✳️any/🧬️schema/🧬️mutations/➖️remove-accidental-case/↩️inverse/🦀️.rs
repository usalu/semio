//! ↩️ `remove-accidental-case` inverse: inserts the removed row back at its position.

use crate::remove_accidental_case::RemoveAccidentalCase;
use crate::insert_accidental_case::InsertAccidentalCase;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &RemoveAccidentalCase, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_accidental_case::InsertAccidentalCase> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveAccidentalCase, base: &En1991Snapshot) -> Option<InsertAccidentalCase> {
    let (index, row) = base.accidental_cases.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertAccidentalCase { index, item: row.clone() })
}
