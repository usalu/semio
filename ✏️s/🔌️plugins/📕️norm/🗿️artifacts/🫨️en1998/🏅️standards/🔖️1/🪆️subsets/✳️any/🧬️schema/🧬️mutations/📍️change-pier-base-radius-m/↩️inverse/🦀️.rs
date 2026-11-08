//! ↩️ `change-pier-base-radius-m` inverse: restores the previous `radius_m`.

use crate::change_pier_base_radius_m::ChangePierBaseRadiusM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangePierBaseRadiusM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pier_base_radius_m::ChangePierBaseRadiusM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePierBaseRadiusM, base: &En1998Snapshot) -> Option<ChangePierBaseRadiusM> {
    let r0 = base.pier_bases.iter().find(|row| row.id == payload.id)?;
    Some(ChangePierBaseRadiusM { id: payload.id.clone(), new_radius_m: r0.radius_m })
}
