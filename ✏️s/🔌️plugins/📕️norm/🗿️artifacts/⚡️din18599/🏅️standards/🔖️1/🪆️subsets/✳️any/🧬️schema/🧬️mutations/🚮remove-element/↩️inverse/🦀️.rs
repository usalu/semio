//! ↩️ `remove-element` inverse: the row back at its position.

use crate::remove_element::RemoveElement;
use crate::insert_element::InsertElement;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(payload: &RemoveElement, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<InsertElement> {
    Ok(base.elements.iter().position(|row| row.id == payload.id).map(|index| vec![Op::from(InsertElement { index, element: base.elements[index].clone() })]).unwrap_or_default())
}
