//! ↩️ `remove-bar-point` inverse: inserts the removed row back at its position.

use crate::remove_bar_point::RemoveBarPoint;
use crate::insert_bar_point::InsertBarPoint;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &RemoveBarPoint, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_bar_point::InsertBarPoint> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveBarPoint, base: &En1992Snapshot) -> Option<InsertBarPoint> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let index = payload.index;
    let row = r0.bars.get(index)?;
    Some(InsertBarPoint { member_id: payload.member_id.clone(), index, bar_point: row.clone() })
}
