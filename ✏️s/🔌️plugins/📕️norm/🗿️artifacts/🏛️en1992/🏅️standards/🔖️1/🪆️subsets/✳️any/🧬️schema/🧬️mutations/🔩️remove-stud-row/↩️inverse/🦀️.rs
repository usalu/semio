//! ↩️ `remove-stud-row` inverse: inserts the removed row back at its position.

use crate::remove_stud_row::RemoveStudRow;
use crate::insert_stud_row::InsertStudRow;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &RemoveStudRow, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_stud_row::InsertStudRow> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveStudRow, base: &En1992Snapshot) -> Option<InsertStudRow> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let index = payload.index;
    let row = r0.stud_rows.get(index)?;
    Some(InsertStudRow { member_id: payload.member_id.clone(), index, stud_row: row.clone() })
}
