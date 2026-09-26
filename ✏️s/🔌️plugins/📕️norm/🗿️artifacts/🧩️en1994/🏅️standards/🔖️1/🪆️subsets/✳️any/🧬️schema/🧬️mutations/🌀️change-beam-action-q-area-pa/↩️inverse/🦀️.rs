//! Inverse for `change-beam-action-q-area-pa`.
use super::ChangeBeamActionQAreaPa;
use crate::{En1994Mutation, En1994Snapshot};
pub fn inverse(payload: &ChangeBeamActionQAreaPa, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    let Some(beam) = base.beams.get(payload.index) else { return Vec::new(); };
    let Some(action) = beam.actions.get(payload.action_index) else { return Vec::new(); };
    vec![En1994Mutation::ChangeBeamActionQAreaPa(ChangeBeamActionQAreaPa { index: payload.index, action_index: payload.action_index, new_q_area_pa: action.q_area_pa })]
}
