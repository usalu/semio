//! ↩️ `remove-tendon-point` inverse: inserts the removed row back at its position.

use crate::remove_tendon_point::RemoveTendonPoint;
use crate::insert_tendon_point::InsertTendonPoint;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &RemoveTendonPoint, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_tendon_point::InsertTendonPoint> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveTendonPoint, base: &En1992Snapshot) -> Option<InsertTendonPoint> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let index = payload.index;
    let row = r0.tendon_points.get(index)?;
    Some(InsertTendonPoint { member_id: payload.member_id.clone(), index, tendon_point: row.clone() })
}
