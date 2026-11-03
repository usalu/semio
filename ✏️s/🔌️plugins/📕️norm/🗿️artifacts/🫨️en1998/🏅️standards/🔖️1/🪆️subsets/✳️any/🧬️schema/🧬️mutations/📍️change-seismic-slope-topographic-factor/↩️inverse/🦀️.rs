//! ↩️ `change-seismic-slope-topographic-factor` inverse: restores the previous `topographic-amplification`.

use crate::change_seismic_slope_topographic_factor::ChangeSeismicSlopeTopographicFactor;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSeismicSlopeTopographicFactor, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_seismic_slope_topographic_factor::ChangeSeismicSlopeTopographicFactor> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSeismicSlopeTopographicFactor, base: &En1998Snapshot) -> Option<ChangeSeismicSlopeTopographicFactor> {
    let r0 = base.slopes.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSeismicSlopeTopographicFactor { id: payload.id.clone(), new_topographic_amplification: r0.topographic_amplification })
}
