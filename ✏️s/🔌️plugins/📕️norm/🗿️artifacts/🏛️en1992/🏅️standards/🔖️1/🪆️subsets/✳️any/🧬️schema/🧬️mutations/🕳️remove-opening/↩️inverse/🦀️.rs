//! ↩️ `remove-opening` inverse: inserts the removed row back at its position.

use crate::remove_opening::RemoveOpening;
use crate::insert_opening::InsertOpening;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &RemoveOpening, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_opening::InsertOpening> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveOpening, base: &En1992Snapshot) -> Option<InsertOpening> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let index = payload.index;
    let row = r0.openings.get(index)?;
    Some(InsertOpening { member_id: payload.member_id.clone(), index, opening: row.clone() })
}
