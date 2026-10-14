//! ↩️ `change-layer-thickness` inverse — restores the layer's `thickness_m`, computed from BASE state; a missing target yields no step.

use crate::change_layer_thickness::ChangeLayerThickness;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(payload: &ChangeLayerThickness, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_layer_thickness::ChangeLayerThickness> {
    Ok((|| {
    base.elements.iter().find(|element| element.id == payload.element_id).and_then(|element| element.layers.get(payload.index)).map(|layer| vec![Op::from(ChangeLayerThickness { element_id: payload.element_id.clone(), index: payload.index, new_thickness_m: layer.thickness_m })]).unwrap_or_default()

    })())
}
