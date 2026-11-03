//! ↩️ `change-storey-stiffness-x` inverse: restores the previous `stiffness_x`.

use crate::change_storey_stiffness_x::ChangeStoreyStiffnessX;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeStoreyStiffnessX, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_storey_stiffness_x::ChangeStoreyStiffnessX> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeStoreyStiffnessX, base: &En1998Snapshot) -> Option<ChangeStoreyStiffnessX> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let r1 = r0.storeys.iter().find(|row| row.id == payload.id)?;
    Some(ChangeStoreyStiffnessX { building_id: payload.building_id.clone(), id: payload.id.clone(), new_stiffness_x: r1.stiffness_x })
}
