//! ↩️ `change-silo-radius-m` inverse: restores the previous `radius_m`.

use crate::change_silo_radius_m::ChangeSiloRadiusM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloRadiusM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_radius_m::ChangeSiloRadiusM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSiloRadiusM, base: &En1998Snapshot) -> Option<ChangeSiloRadiusM> {
    let r0 = base.silos.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSiloRadiusM { id: payload.id.clone(), new_radius_m: r0.radius_m })
}
