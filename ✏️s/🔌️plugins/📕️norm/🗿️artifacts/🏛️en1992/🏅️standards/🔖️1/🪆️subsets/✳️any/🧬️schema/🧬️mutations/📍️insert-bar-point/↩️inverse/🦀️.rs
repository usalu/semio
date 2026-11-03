//! ↩️ `insert-bar-point` inverse: removes the inserted row again.

use crate::insert_bar_point::InsertBarPoint;
use crate::remove_bar_point::RemoveBarPoint;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &InsertBarPoint, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_bar_point::RemoveBarPoint> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertBarPoint, base: &En1992Snapshot) -> Option<RemoveBarPoint> {
    let _r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let index = payload.index;
    Some(RemoveBarPoint { member_id: payload.member_id.clone(), index })
}
