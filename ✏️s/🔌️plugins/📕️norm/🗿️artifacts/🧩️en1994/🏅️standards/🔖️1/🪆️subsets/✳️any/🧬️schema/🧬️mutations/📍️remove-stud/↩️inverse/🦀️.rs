//! ↩️ `remove-stud` inverse: inserts the removed row back at its position.

use crate::remove_stud::RemoveStud;
use crate::insert_stud::InsertStud;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;

pub fn inverse<Op>(payload: &RemoveStud, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_stud::InsertStud> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveStud, base: &En1994Snapshot) -> Option<InsertStud> {
    let index = base.studs.iter().position(|row| row.id == payload.id)?;
    let row = base.studs.get(index)?;
    Some(InsertStud { index, item: row.clone() })
}
