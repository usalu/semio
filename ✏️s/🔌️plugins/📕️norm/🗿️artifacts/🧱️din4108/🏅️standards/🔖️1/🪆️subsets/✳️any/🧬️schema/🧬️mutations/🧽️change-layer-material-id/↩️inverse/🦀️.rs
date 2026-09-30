//! ↩️ `change-layer-material-id` inverse — restores the layer's `material_id`, computed from BASE state; a missing target yields no step.

use super::ChangeLayerMaterialId;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(payload: &ChangeLayerMaterialId, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    base.elements.iter().find(|element| element.id == payload.element_id).and_then(|element| element.layers.get(payload.index)).map(|layer| vec![Din4108Mutation::ChangeLayerMaterialId(ChangeLayerMaterialId { element_id: payload.element_id.clone(), index: payload.index, new_material_id: layer.material_id.clone() })]).unwrap_or_default()
}
