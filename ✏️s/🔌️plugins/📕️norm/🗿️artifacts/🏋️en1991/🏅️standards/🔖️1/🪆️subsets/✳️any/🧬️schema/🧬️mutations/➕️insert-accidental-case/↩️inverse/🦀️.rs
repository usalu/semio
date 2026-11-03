//! ↩️ `insert-accidental-case` inverse: removes the inserted row again.

use crate::insert_accidental_case::InsertAccidentalCase;
use crate::remove_accidental_case::RemoveAccidentalCase;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &InsertAccidentalCase, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_accidental_case::RemoveAccidentalCase> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertAccidentalCase, base: &En1991Snapshot) -> Option<RemoveAccidentalCase> {
    Some(RemoveAccidentalCase { id: payload.item.id.clone() })
}
