//! ↩️ `change-tendon-point-x` inverse: restores the previous `x`.

use crate::change_tendon_point_x::ChangeTendonPointX;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeTendonPointX, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tendon_point_x::ChangeTendonPointX> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTendonPointX, base: &En1992Snapshot) -> Option<ChangeTendonPointX> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let r1 = r0.tendon_points.iter().find(|row| row.id == payload.tendon_point_id)?;
    Some(ChangeTendonPointX { member_id: payload.member_id.clone(), tendon_point_id: payload.tendon_point_id.clone(), new_x: r1.x })
}
