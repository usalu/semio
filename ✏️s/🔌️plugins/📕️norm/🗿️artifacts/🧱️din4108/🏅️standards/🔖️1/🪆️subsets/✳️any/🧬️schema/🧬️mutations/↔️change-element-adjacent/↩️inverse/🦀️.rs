//! ↩️ `change-element-adjacent` inverse — restores the element's `adjacent`, computed from BASE state; a missing target yields no step.

use super::ChangeElementAdjacent;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(payload: &ChangeElementAdjacent, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    base.elements.iter().find(|element| element.id == payload.element_id).map(|element| vec![Din4108Mutation::ChangeElementAdjacent(ChangeElementAdjacent { element_id: payload.element_id.clone(), new_adjacent: element.adjacent.clone() })]).unwrap_or_default()
}
