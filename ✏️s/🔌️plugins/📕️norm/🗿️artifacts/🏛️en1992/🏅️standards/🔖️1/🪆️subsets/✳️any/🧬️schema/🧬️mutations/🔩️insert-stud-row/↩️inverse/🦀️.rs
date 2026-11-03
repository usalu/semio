//! ↩️ `insert-stud-row` inverse: removes the inserted row again.

use crate::insert_stud_row::InsertStudRow;
use crate::remove_stud_row::RemoveStudRow;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &InsertStudRow, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_stud_row::RemoveStudRow> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertStudRow, base: &En1992Snapshot) -> Option<RemoveStudRow> {
    let _r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let index = payload.index;
    Some(RemoveStudRow { member_id: payload.member_id.clone(), index })
}
