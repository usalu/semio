//! ↩️ `change-tendon-point-z` inverse: restores the previous `z`.

use crate::change_tendon_point_z::ChangeTendonPointZ;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeTendonPointZ, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tendon_point_z::ChangeTendonPointZ> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTendonPointZ, base: &En1992Snapshot) -> Option<ChangeTendonPointZ> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let r1 = r0.tendon_points.iter().find(|row| row.id == payload.tendon_point_id)?;
    Some(ChangeTendonPointZ { member_id: payload.member_id.clone(), tendon_point_id: payload.tendon_point_id.clone(), new_z: r1.z })
}
