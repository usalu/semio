//! ↩️ `change-layer-poisson-ratio` inverse: restores the previous `poisson_ratio`.

use crate::change_layer_poisson_ratio::ChangeLayerPoissonRatio;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeLayerPoissonRatio, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_layer_poisson_ratio::ChangeLayerPoissonRatio> {
    Ok(match base.layers.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeLayerPoissonRatio { id: payload.id.clone(), new_poisson_ratio: row.poisson_ratio })],
        None => Vec::new(),
    })
}
