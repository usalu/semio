//! ↩️ `change-seismic-slope-circle-centre-z` inverse: restores the previous `circle-centre-z`.

use crate::change_seismic_slope_circle_centre_z::ChangeSeismicSlopeCircleCentreZ;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSeismicSlopeCircleCentreZ, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_seismic_slope_circle_centre_z::ChangeSeismicSlopeCircleCentreZ> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSeismicSlopeCircleCentreZ, base: &En1998Snapshot) -> Option<ChangeSeismicSlopeCircleCentreZ> {
    let r0 = base.slopes.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSeismicSlopeCircleCentreZ { id: payload.id.clone(), new_circle_centre_z: r0.circle_centre_z })
}
