//! ↩️ `remove-layer` inverse — re-inserts the removed layer at its position, computed from BASE state; a missing target yields no step.

use crate::remove_layer::RemoveLayer;
use crate::insert_layer::InsertLayer;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(payload: &RemoveLayer, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_layer::InsertLayer> {
    Ok((|| {
    base.elements.iter().find(|element| element.id == payload.element_id).and_then(|element| element.layers.get(payload.index)).map(|layer| vec![Op::from(InsertLayer { element_id: payload.element_id.clone(), index: Some(payload.index), layer: layer.clone() })]).unwrap_or_default()

    })())
}
