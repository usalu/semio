//! ↩️ `change-element-u` inverse.

use crate::change_element_u::ChangeElementU;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(payload: &ChangeElementU, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_element_u::ChangeElementU> {
    Ok((|| {
    let current = base.elements.iter().find(|e| e.id == payload.element_id).map(|e| e.u_value_w_m2k).unwrap_or(payload.new_u_value_w_m2k);
    vec![Op::from(ChangeElementU { element_id: payload.element_id.clone(), new_u_value_w_m2k: current })]

    })())
}
