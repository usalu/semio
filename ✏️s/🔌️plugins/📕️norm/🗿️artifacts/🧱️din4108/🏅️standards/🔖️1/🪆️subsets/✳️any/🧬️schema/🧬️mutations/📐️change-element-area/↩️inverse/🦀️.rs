//! ↩️ `change-element-area` inverse — restores the element's `area_m2`, computed from BASE state; a missing target yields no step.

use super::ChangeElementArea;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(payload: &ChangeElementArea, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    base.elements.iter().find(|element| element.id == payload.element_id).map(|element| vec![Din4108Mutation::ChangeElementArea(ChangeElementArea { element_id: payload.element_id.clone(), new_area_m2: element.area_m2 })]).unwrap_or_default()
}
