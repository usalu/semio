//! ↩️ `change-layer-compressive-class` inverse — restores the layer's `compressive_class`, computed from BASE state; a missing target yields no step.

use super::ChangeLayerCompressiveClass;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(payload: &ChangeLayerCompressiveClass, base: &Din4108Snapshot) -> Result<Vec<Din4108Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.elements.iter().find(|element| element.id == payload.element_id).and_then(|element| element.layers.get(payload.index)).map(|layer| vec![Din4108Mutation::ChangeLayerCompressiveClass(ChangeLayerCompressiveClass { element_id: payload.element_id.clone(), index: payload.index, new_compressive_class: layer.compressive_class.clone() })]).unwrap_or_default()

    })())
}
