//! ↩️ `insert-stud` inverse: removes the inserted row again.

use crate::insert_stud::InsertStud;
use crate::remove_stud::RemoveStud;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;

pub fn inverse<Op>(payload: &InsertStud, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_stud::RemoveStud> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertStud, _base: &En1994Snapshot) -> Option<RemoveStud> {
    Some(RemoveStud { id: payload.item.id.clone() })
}
