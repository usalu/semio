//! ↩️ `insert-element` inverse — removes the inserted element at its landing position, computed from BASE state; a missing target yields no step.

use super::InsertElement;
use crate::mutations::remove_element::RemoveElement;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(payload: &InsertElement, base: &Din4108Snapshot) -> Result<Vec<Din4108Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Din4108Mutation::RemoveElement(RemoveElement { index: payload.index.min(base.elements.len()) })]

    })())
}
