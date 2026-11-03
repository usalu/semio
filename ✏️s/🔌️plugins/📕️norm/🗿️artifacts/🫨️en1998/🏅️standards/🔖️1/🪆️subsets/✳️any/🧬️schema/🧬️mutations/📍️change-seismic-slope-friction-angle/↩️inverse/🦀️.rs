//! ↩️ `change-seismic-slope-friction-angle` inverse: restores the previous `friction-angle`.

use crate::change_seismic_slope_friction_angle::ChangeSeismicSlopeFrictionAngle;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSeismicSlopeFrictionAngle, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_seismic_slope_friction_angle::ChangeSeismicSlopeFrictionAngle> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSeismicSlopeFrictionAngle, base: &En1998Snapshot) -> Option<ChangeSeismicSlopeFrictionAngle> {
    let r0 = base.slopes.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSeismicSlopeFrictionAngle { id: payload.id.clone(), new_friction_angle: r0.friction_angle })
}
