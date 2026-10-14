//! ↩️ `change-layer-mu` inverse — restores the layer's `mu`, computed from BASE state; a missing target yields no step.

use crate::change_layer_mu::ChangeLayerMu;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(payload: &ChangeLayerMu, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_layer_mu::ChangeLayerMu> {
    Ok((|| {
    base.elements.iter().find(|element| element.id == payload.element_id).and_then(|element| element.layers.get(payload.index)).map(|layer| vec![Op::from(ChangeLayerMu { element_id: payload.element_id.clone(), index: payload.index, new_mu: layer.mu })]).unwrap_or_default()

    })())
}
