//! ↩️ `remove-element` inverse — re-inserts the removed element at its position, computed from BASE state; a missing target yields no step.

use super::RemoveElement;
use crate::mutations::insert_element::InsertElement;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(payload: &RemoveElement, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    base.elements.get(payload.index).map(|element| vec![Din4108Mutation::InsertElement(InsertElement { index: payload.index, element: element.clone() })]).unwrap_or_default()
}
