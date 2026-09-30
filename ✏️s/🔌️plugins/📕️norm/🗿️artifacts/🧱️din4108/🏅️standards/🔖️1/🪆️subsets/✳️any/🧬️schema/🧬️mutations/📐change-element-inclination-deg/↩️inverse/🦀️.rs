//! ↩️ `change-element-inclination-deg` inverse — restores the element's `inclination_deg`, computed from BASE state; a missing target yields no step.

use super::ChangeElementInclinationDeg;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(payload: &ChangeElementInclinationDeg, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    base.elements.iter().find(|element| element.id == payload.element_id).map(|element| vec![Din4108Mutation::ChangeElementInclinationDeg(ChangeElementInclinationDeg { element_id: payload.element_id.clone(), new_inclination_deg: element.inclination_deg })]).unwrap_or_default()
}
