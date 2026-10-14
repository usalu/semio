//! ↩️ `change-element-delta-ur` inverse — restores the element's `delta_u_r`, computed from BASE state; a missing target yields no step.

use crate::change_element_delta_u_r::ChangeElementDeltaUr;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(payload: &ChangeElementDeltaUr, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_element_delta_u_r::ChangeElementDeltaUr> {
    Ok((|| {
    base.elements.iter().find(|element| element.id == payload.element_id).map(|element| vec![Op::from(ChangeElementDeltaUr { element_id: payload.element_id.clone(), new_delta_u_r: element.delta_u_r })]).unwrap_or_default()

    })())
}
