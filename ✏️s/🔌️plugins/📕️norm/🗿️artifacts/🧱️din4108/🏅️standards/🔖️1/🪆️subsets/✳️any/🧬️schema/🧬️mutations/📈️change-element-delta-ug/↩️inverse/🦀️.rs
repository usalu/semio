//! ↩️ `change-element-delta-ug` inverse — restores the element's `delta_u_g`, computed from BASE state; a missing target yields no step.

use super::ChangeElementDeltaUg;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(payload: &ChangeElementDeltaUg, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    base.elements.iter().find(|element| element.id == payload.element_id).map(|element| vec![Din4108Mutation::ChangeElementDeltaUg(ChangeElementDeltaUg { element_id: payload.element_id.clone(), new_delta_u_g: element.delta_u_g })]).unwrap_or_default()
}
