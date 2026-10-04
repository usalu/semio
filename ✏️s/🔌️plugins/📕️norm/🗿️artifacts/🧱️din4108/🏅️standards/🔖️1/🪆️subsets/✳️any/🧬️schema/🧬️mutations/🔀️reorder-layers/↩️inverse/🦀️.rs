//! ↩️ `reorder-layers` inverse — moves the layer back to its original position, computed from BASE state; a missing target yields no step.

use super::ReorderLayers;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(payload: &ReorderLayers, base: &Din4108Snapshot) -> Result<Vec<Din4108Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.elements.iter().find(|element| element.id == payload.element_id).filter(|element| payload.from < element.layers.len() && payload.to < element.layers.len()).map(|_| vec![Din4108Mutation::ReorderLayers(ReorderLayers { element_id: payload.element_id.clone(), from: payload.to, to: payload.from })]).unwrap_or_default()

    })())
}
