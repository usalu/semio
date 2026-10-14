//! ↩️ `reorder-layers` inverse — moves the layer back to its original position, computed from BASE state; a missing target yields no step.

use crate::reorder_layers::ReorderLayers;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(payload: &ReorderLayers, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::reorder_layers::ReorderLayers> {
    Ok((|| {
    base.elements.iter().find(|element| element.id == payload.element_id).filter(|element| payload.from < element.layers.len() && payload.to < element.layers.len()).map(|_| vec![Op::from(ReorderLayers { element_id: payload.element_id.clone(), from: payload.to, to: payload.from })]).unwrap_or_default()

    })())
}
