//! ↩️ `remove-layer` inverse — re-inserts the removed layer at its position, computed from BASE state; a missing target yields no step.

use super::RemoveLayer;
use crate::mutations::insert_layer::InsertLayer;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(payload: &RemoveLayer, base: &Din4108Snapshot) -> Result<Vec<Din4108Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.elements.iter().find(|element| element.id == payload.element_id).and_then(|element| element.layers.get(payload.index)).map(|layer| vec![Din4108Mutation::InsertLayer(InsertLayer { element_id: payload.element_id.clone(), index: payload.index, layer: layer.clone() })]).unwrap_or_default()

    })())
}
