//! ↩️ `change-seismic-slope-angle` inverse: restores the previous `angle`.

use crate::change_seismic_slope_angle::ChangeSeismicSlopeAngle;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSeismicSlopeAngle, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_seismic_slope_angle::ChangeSeismicSlopeAngle> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSeismicSlopeAngle, base: &En1998Snapshot) -> Option<ChangeSeismicSlopeAngle> {
    let r0 = base.slopes.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSeismicSlopeAngle { id: payload.id.clone(), new_angle: r0.angle })
}
