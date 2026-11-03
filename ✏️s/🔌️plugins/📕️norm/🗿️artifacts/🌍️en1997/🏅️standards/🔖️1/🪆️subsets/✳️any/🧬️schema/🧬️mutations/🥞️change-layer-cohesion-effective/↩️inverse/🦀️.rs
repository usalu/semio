//! ↩️ `change-layer-cohesion-effective` inverse: restores the previous `cohesion_effective`.

use crate::change_layer_cohesion_effective::ChangeLayerCohesionEffective;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeLayerCohesionEffective, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_layer_cohesion_effective::ChangeLayerCohesionEffective> {
    Ok(match base.layers.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeLayerCohesionEffective { id: payload.id.clone(), new_cohesion_effective: row.cohesion_effective })],
        None => Vec::new(),
    })
}
