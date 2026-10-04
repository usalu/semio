//! ↩️ `change-layer-thickness` inverse — restores the layer's `thickness_m`, computed from BASE state; a missing target yields no step.

use super::ChangeLayerThickness;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(payload: &ChangeLayerThickness, base: &Din4108Snapshot) -> Result<Vec<Din4108Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.elements.iter().find(|element| element.id == payload.element_id).and_then(|element| element.layers.get(payload.index)).map(|layer| vec![Din4108Mutation::ChangeLayerThickness(ChangeLayerThickness { element_id: payload.element_id.clone(), index: payload.index, new_thickness_m: layer.thickness_m })]).unwrap_or_default()

    })())
}
