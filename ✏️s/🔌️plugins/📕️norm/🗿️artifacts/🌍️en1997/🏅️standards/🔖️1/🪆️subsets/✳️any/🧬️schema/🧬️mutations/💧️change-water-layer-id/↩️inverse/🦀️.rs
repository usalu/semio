//! ↩️ `change-water-layer-id` inverse: restores the previous `layer_id`.

use crate::change_water_layer_id::ChangeWaterLayerId;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeWaterLayerId, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_water_layer_id::ChangeWaterLayerId> {
    Ok(match base.layer_waters.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWaterLayerId { id: payload.id.clone(), new_layer_id: row.layer_id.clone() })],
        None => Vec::new(),
    })
}
