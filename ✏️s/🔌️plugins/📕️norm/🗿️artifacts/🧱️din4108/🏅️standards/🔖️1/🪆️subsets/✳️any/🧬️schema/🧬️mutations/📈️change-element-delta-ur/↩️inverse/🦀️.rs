//! ↩️ `change-element-delta-ur` inverse — restores the element's `delta_u_r`, computed from BASE state; a missing target yields no step.

use super::ChangeElementDeltaUr;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(payload: &ChangeElementDeltaUr, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    base.elements.iter().find(|element| element.id == payload.element_id).map(|element| vec![Din4108Mutation::ChangeElementDeltaUr(ChangeElementDeltaUr { element_id: payload.element_id.clone(), new_delta_u_r: element.delta_u_r })]).unwrap_or_default()
}
