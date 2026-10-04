//! ↩️ `change-element-orientation-deg` inverse — restores the element's `orientation_deg`, computed from BASE state; a missing target yields no step.

use super::ChangeElementOrientationDeg;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(payload: &ChangeElementOrientationDeg, base: &Din4108Snapshot) -> Result<Vec<Din4108Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.elements.iter().find(|element| element.id == payload.element_id).map(|element| vec![Din4108Mutation::ChangeElementOrientationDeg(ChangeElementOrientationDeg { element_id: payload.element_id.clone(), new_orientation_deg: element.orientation_deg })]).unwrap_or_default()

    })())
}
