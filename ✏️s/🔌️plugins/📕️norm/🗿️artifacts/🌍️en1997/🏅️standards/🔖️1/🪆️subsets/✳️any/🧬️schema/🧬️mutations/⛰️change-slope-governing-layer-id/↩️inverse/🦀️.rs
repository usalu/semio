//! ↩️ `change-slope-governing-layer-id` inverse: restores the previous `governing_layer_id`.

use crate::change_slope_governing_layer_id::ChangeSlopeGoverningLayerId;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeSlopeGoverningLayerId, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_slope_governing_layer_id::ChangeSlopeGoverningLayerId> {
    Ok(match base.slopes.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSlopeGoverningLayerId { id: payload.id.clone(), new_governing_layer_id: row.governing_layer_id.clone() })],
        None => Vec::new(),
    })
}
