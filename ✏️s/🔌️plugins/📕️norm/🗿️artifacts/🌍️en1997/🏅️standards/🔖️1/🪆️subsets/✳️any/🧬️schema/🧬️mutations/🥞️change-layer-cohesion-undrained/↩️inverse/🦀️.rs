//! ↩️ `change-layer-cohesion-undrained` inverse: restores the previous `cohesion_undrained`.

use crate::change_layer_cohesion_undrained::ChangeLayerCohesionUndrained;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeLayerCohesionUndrained, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_layer_cohesion_undrained::ChangeLayerCohesionUndrained> {
    Ok(match base.layers.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeLayerCohesionUndrained { id: payload.id.clone(), new_cohesion_undrained: row.cohesion_undrained })],
        None => Vec::new(),
    })
}
