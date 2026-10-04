//! ↩️ `insert-layer` inverse — removes the inserted layer at its landing position, computed from BASE state; a missing target yields no step.

use super::InsertLayer;
use crate::mutations::remove_layer::RemoveLayer;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(payload: &InsertLayer, base: &Din4108Snapshot) -> Result<Vec<Din4108Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.elements.iter().find(|element| element.id == payload.element_id).map(|element| vec![Din4108Mutation::RemoveLayer(RemoveLayer { element_id: payload.element_id.clone(), index: payload.index.min(element.layers.len()) })]).unwrap_or_default()

    })())
}
