//! ↩️ `change-seismic-slope-cohesion` inverse: restores the previous `cohesion`.

use crate::change_seismic_slope_cohesion::ChangeSeismicSlopeCohesion;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSeismicSlopeCohesion, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_seismic_slope_cohesion::ChangeSeismicSlopeCohesion> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSeismicSlopeCohesion, base: &En1998Snapshot) -> Option<ChangeSeismicSlopeCohesion> {
    let r0 = base.slopes.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSeismicSlopeCohesion { id: payload.id.clone(), new_cohesion: r0.cohesion })
}
