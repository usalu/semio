//! ↩️ `change-seismic-slope-circle-radius` inverse: restores the previous `circle-radius`.

use crate::change_seismic_slope_circle_radius::ChangeSeismicSlopeCircleRadius;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSeismicSlopeCircleRadius, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_seismic_slope_circle_radius::ChangeSeismicSlopeCircleRadius> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSeismicSlopeCircleRadius, base: &En1998Snapshot) -> Option<ChangeSeismicSlopeCircleRadius> {
    let r0 = base.slopes.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSeismicSlopeCircleRadius { id: payload.id.clone(), new_circle_radius: r0.circle_radius })
}
