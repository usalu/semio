//! ↩️ `change-layer-application-type` inverse — restores the layer's `application_type`, computed from BASE state; a missing target yields no step.

use crate::change_layer_application_type::ChangeLayerApplicationType;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(payload: &ChangeLayerApplicationType, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_layer_application_type::ChangeLayerApplicationType> {
    Ok((|| {
    base.elements.iter().find(|element| element.id == payload.element_id).and_then(|element| element.layers.get(payload.index)).map(|layer| vec![Op::from(ChangeLayerApplicationType { element_id: payload.element_id.clone(), index: payload.index, new_application_type: layer.application_type.clone() })]).unwrap_or_default()

    })())
}
