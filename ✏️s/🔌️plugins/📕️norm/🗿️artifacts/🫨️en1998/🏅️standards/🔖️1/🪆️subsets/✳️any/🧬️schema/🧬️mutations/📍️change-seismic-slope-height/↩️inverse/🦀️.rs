//! ↩️ `change-seismic-slope-height` inverse: restores the previous `height`.

use crate::change_seismic_slope_height::ChangeSeismicSlopeHeight;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSeismicSlopeHeight, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_seismic_slope_height::ChangeSeismicSlopeHeight> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSeismicSlopeHeight, base: &En1998Snapshot) -> Option<ChangeSeismicSlopeHeight> {
    let r0 = base.slopes.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSeismicSlopeHeight { id: payload.id.clone(), new_height: r0.height })
}
