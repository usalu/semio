//! ↩️ `change-layer-gamma` inverse: restores the previous `gamma`.

use crate::change_layer_gamma::ChangeLayerGamma;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeLayerGamma, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_layer_gamma::ChangeLayerGamma> {
    Ok(match base.layers.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeLayerGamma { id: payload.id.clone(), new_gamma: row.gamma })],
        None => Vec::new(),
    })
}
