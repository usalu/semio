//! ↩️ `change-element-delta-uf` inverse — restores the element's `delta_u_f`, computed from BASE state; a missing target yields no step.

use super::ChangeElementDeltaUf;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(payload: &ChangeElementDeltaUf, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    base.elements.iter().find(|element| element.id == payload.element_id).map(|element| vec![Din4108Mutation::ChangeElementDeltaUf(ChangeElementDeltaUf { element_id: payload.element_id.clone(), new_delta_u_f: element.delta_u_f })]).unwrap_or_default()
}
