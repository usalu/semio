//! ↩️ `change-layer-soil-type` inverse: restores the previous `soil_type`.

use crate::change_layer_soil_type::ChangeLayerSoilType;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeLayerSoilType, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_layer_soil_type::ChangeLayerSoilType> {
    Ok(match base.layers.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeLayerSoilType { id: payload.id.clone(), new_soil_type: row.soil_type })],
        None => Vec::new(),
    })
}
