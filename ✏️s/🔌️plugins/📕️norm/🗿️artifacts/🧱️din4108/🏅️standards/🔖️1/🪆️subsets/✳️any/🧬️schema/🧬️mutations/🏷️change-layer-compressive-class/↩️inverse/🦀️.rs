//! ↩️ `change-layer-compressive-class` inverse — restores the layer's `compressive_class`, computed from BASE state; a missing target yields no step.

use crate::change_layer_compressive_class::ChangeLayerCompressiveClass;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(payload: &ChangeLayerCompressiveClass, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_layer_compressive_class::ChangeLayerCompressiveClass> {
    Ok((|| {
    base.elements.iter().find(|element| element.id == payload.element_id).and_then(|element| element.layers.get(payload.index)).map(|layer| vec![Op::from(ChangeLayerCompressiveClass { element_id: payload.element_id.clone(), index: payload.index, new_compressive_class: layer.compressive_class.clone() })]).unwrap_or_default()

    })())
}
