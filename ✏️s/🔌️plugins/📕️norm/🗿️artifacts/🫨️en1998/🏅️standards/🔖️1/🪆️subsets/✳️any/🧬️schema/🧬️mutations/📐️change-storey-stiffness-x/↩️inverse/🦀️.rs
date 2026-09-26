//! Inverse for `change-storey-stiffness-x`.
use super::ChangeStoreyStiffnessX;
use crate::{En1998Mutation, En1998Snapshot};

pub fn inverse(payload: &ChangeStoreyStiffnessX, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    match base.buildings.get(payload.building_index).and_then(|b| b.storeys.get(payload.storey_index)) {
        Some(st) => vec![En1998Mutation::ChangeStoreyStiffnessX(ChangeStoreyStiffnessX { building_index: payload.building_index, storey_index: payload.storey_index, new_stiffness_x: st.stiffness_x })],
        None => Vec::new(),
    }
}
