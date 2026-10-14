//! ↩️ `remove-element` inverse — re-inserts the removed element at its position, computed from BASE state; a missing target yields no step.

use crate::remove_element::RemoveElement;
use crate::insert_element::InsertElement;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(payload: &RemoveElement, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_element::InsertElement> {
    Ok((|| {
    base.elements.get(payload.index).map(|element| vec![Op::from(InsertElement { index: Some(payload.index), element: element.clone() })]).unwrap_or_default()

    })())
}
