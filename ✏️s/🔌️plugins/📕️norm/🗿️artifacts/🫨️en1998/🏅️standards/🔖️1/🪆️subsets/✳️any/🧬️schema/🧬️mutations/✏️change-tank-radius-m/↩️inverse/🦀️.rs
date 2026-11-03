//! ↩️ `change-tank-radius-m` inverse: restores the previous `radius_m`.

use crate::change_tank_radius_m::ChangeTankRadiusM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeTankRadiusM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tank_radius_m::ChangeTankRadiusM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTankRadiusM, base: &En1998Snapshot) -> Option<ChangeTankRadiusM> {
    let r0 = base.tanks.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTankRadiusM { id: payload.id.clone(), new_radius_m: r0.radius_m })
}
