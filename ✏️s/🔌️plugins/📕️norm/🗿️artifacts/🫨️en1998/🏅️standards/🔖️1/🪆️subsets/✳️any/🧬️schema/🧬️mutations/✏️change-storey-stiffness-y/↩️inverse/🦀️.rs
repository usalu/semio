//! ↩️ `change-storey-stiffness-y` inverse: restores the previous `stiffness_y`.

use crate::change_storey_stiffness_y::ChangeStoreyStiffnessY;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeStoreyStiffnessY, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_storey_stiffness_y::ChangeStoreyStiffnessY> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeStoreyStiffnessY, base: &En1998Snapshot) -> Option<ChangeStoreyStiffnessY> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let r1 = r0.storeys.iter().find(|row| row.id == payload.id)?;
    Some(ChangeStoreyStiffnessY { building_id: payload.building_id.clone(), id: payload.id.clone(), new_stiffness_y: r1.stiffness_y })
}
