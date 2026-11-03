//! ↩️ `change-layer-gamma-prime` inverse: restores the previous `gamma_prime`.

use crate::change_layer_gamma_prime::ChangeLayerGammaPrime;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeLayerGammaPrime, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_layer_gamma_prime::ChangeLayerGammaPrime> {
    Ok(match base.layers.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeLayerGammaPrime { id: payload.id.clone(), new_gamma_prime: row.gamma_prime })],
        None => Vec::new(),
    })
}
