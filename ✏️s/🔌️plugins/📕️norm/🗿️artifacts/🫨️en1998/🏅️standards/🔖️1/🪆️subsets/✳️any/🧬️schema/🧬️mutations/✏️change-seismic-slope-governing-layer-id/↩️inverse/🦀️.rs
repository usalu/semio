//! ↩️ `change-seismic-slope-governing-layer-id` inverse: restores the previous `governing_layer_id`.

use crate::change_seismic_slope_governing_layer_id::ChangeSeismicSlopeGoverningLayerId;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSeismicSlopeGoverningLayerId, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_seismic_slope_governing_layer_id::ChangeSeismicSlopeGoverningLayerId> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSeismicSlopeGoverningLayerId, base: &En1998Snapshot) -> Option<ChangeSeismicSlopeGoverningLayerId> {
    let r0 = base.slopes.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSeismicSlopeGoverningLayerId { id: payload.id.clone(), new_governing_layer_id: r0.governing_layer_id.clone() })
}
