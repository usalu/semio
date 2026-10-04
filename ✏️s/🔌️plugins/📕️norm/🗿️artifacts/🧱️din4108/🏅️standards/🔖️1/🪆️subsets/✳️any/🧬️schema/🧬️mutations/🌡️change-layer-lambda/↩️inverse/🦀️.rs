//! ↩️ `change-layer-lambda` inverse — restores the layer's `lambda`, computed from BASE state; a missing target yields no step.

use super::ChangeLayerLambda;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(payload: &ChangeLayerLambda, base: &Din4108Snapshot) -> Result<Vec<Din4108Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.elements.iter().find(|element| element.id == payload.element_id).and_then(|element| element.layers.get(payload.index)).map(|layer| vec![Din4108Mutation::ChangeLayerLambda(ChangeLayerLambda { element_id: payload.element_id.clone(), index: payload.index, new_lambda: layer.lambda })]).unwrap_or_default()

    })())
}
