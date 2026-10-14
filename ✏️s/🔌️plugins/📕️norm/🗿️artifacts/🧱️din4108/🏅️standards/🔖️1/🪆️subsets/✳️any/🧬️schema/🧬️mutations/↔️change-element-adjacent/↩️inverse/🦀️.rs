//! ↩️ `change-element-adjacent` inverse — restores the element's `adjacent`, computed from BASE state; a missing target yields no step.

use crate::change_element_adjacent::ChangeElementAdjacent;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(payload: &ChangeElementAdjacent, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_element_adjacent::ChangeElementAdjacent> {
    Ok((|| {
    base.elements.iter().find(|element| element.id == payload.element_id).map(|element| vec![Op::from(ChangeElementAdjacent { element_id: payload.element_id.clone(), new_adjacent: element.adjacent.clone() })]).unwrap_or_default()

    })())
}
