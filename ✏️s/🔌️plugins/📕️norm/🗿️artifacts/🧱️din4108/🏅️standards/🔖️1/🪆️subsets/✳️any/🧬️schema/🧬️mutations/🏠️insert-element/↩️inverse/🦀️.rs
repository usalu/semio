//! ↩️ `insert-element` inverse — removes the inserted element at its landing position, computed from BASE state; a missing target yields no step.

use crate::insert_element::InsertElement;
use crate::remove_element::RemoveElement;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(payload: &InsertElement, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_element::RemoveElement> {
    Ok((|| {
    vec![Op::from(RemoveElement { index: payload.index.unwrap_or(usize::MAX).min(base.elements.len()) })]

    })())
}
