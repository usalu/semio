//! ↩️ `change-layer-application-type` inverse — restores the layer's `application_type`, computed from BASE state; a missing target yields no step.

use super::ChangeLayerApplicationType;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(payload: &ChangeLayerApplicationType, base: &Din4108Snapshot) -> Result<Vec<Din4108Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.elements.iter().find(|element| element.id == payload.element_id).and_then(|element| element.layers.get(payload.index)).map(|layer| vec![Din4108Mutation::ChangeLayerApplicationType(ChangeLayerApplicationType { element_id: payload.element_id.clone(), index: payload.index, new_application_type: layer.application_type.clone() })]).unwrap_or_default()

    })())
}
