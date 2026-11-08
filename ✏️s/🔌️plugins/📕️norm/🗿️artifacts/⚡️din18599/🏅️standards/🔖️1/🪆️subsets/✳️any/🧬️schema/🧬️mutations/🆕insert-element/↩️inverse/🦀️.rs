//! ↩️ `insert-element` inverse: the removal of the inserted row.

use crate::insert_element::InsertElement;
use crate::remove_element::RemoveElement;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(payload: &InsertElement, _base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<RemoveElement> {
    Ok(vec![Op::from(RemoveElement { id: payload.element.id.clone() })])
}
