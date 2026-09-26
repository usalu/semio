//! Inverse for `change-slab-action-q-area-pa`.
use super::ChangeSlabActionQAreaPa;
use crate::{En1994Mutation, En1994Snapshot};
pub fn inverse(payload: &ChangeSlabActionQAreaPa, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    let Some(slab) = base.slabs.get(payload.index) else { return Vec::new(); };
    let Some(action) = slab.actions.get(payload.action_index) else { return Vec::new(); };
    vec![En1994Mutation::ChangeSlabActionQAreaPa(ChangeSlabActionQAreaPa { index: payload.index, action_index: payload.action_index, new_q_area_pa: action.q_area_pa })]
}
