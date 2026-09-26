//! ↩️ `change-element-u` inverse.

use crate::mutations::change_element_u::ChangeElementU;
use crate::mutations::Din18599Mutation;
use crate::Din18599Snapshot;

pub fn inverse(payload: &ChangeElementU, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
    let current = base.elements.iter().find(|e| e.id == payload.element_id).map(|e| e.u_value_w_m2k).unwrap_or(payload.new_u_value_w_m2k);
    vec![Din18599Mutation::ChangeElementU(ChangeElementU { element_id: payload.element_id.clone(), new_u_value_w_m2k: current })]
}
