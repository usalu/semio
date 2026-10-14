//! ↩️ `insert-layer` inverse — removes the inserted layer at its landing position, computed from BASE state; a missing target yields no step.

use crate::insert_layer::InsertLayer;
use crate::remove_layer::RemoveLayer;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(payload: &InsertLayer, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_layer::RemoveLayer> {
    Ok((|| {
    base.elements.iter().find(|element| element.id == payload.element_id).map(|element| vec![Op::from(RemoveLayer { element_id: payload.element_id.clone(), index: payload.index.unwrap_or(usize::MAX).min(element.layers.len()) })]).unwrap_or_default()

    })())
}
