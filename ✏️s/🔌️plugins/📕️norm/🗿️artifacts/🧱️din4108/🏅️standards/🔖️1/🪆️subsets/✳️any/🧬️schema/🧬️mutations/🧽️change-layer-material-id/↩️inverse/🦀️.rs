//! ↩️ `change-layer-material-id` inverse — restores the layer's `material_id`, computed from BASE state; a missing target yields no step.

use crate::change_layer_material_id::ChangeLayerMaterialId;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(payload: &ChangeLayerMaterialId, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_layer_material_id::ChangeLayerMaterialId> {
    Ok((|| {
    base.elements.iter().find(|element| element.id == payload.element_id).and_then(|element| element.layers.get(payload.index)).map(|layer| vec![Op::from(ChangeLayerMaterialId { element_id: payload.element_id.clone(), index: payload.index, new_material_id: layer.material_id.clone() })]).unwrap_or_default()

    })())
}
