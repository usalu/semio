//! ↩️ `insert-tendon-point` inverse: removes the inserted row again.

use crate::insert_tendon_point::InsertTendonPoint;
use crate::remove_tendon_point::RemoveTendonPoint;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &InsertTendonPoint, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_tendon_point::RemoveTendonPoint> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertTendonPoint, base: &En1992Snapshot) -> Option<RemoveTendonPoint> {
    let _r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let index = payload.index;
    Some(RemoveTendonPoint { member_id: payload.member_id.clone(), index })
}
