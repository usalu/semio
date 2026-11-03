//! ↩️ `change-seismic-slope-circle-centre-x` inverse: restores the previous `circle-centre-x`.

use crate::change_seismic_slope_circle_centre_x::ChangeSeismicSlopeCircleCentreX;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSeismicSlopeCircleCentreX, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_seismic_slope_circle_centre_x::ChangeSeismicSlopeCircleCentreX> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSeismicSlopeCircleCentreX, base: &En1998Snapshot) -> Option<ChangeSeismicSlopeCircleCentreX> {
    let r0 = base.slopes.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSeismicSlopeCircleCentreX { id: payload.id.clone(), new_circle_centre_x: r0.circle_centre_x })
}
