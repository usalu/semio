//! Inverse for `change-storey-drift-x-m`.
use super::ChangeStoreyDriftXM;
use crate::{En1998Mutation, En1998Snapshot};

pub fn inverse(payload: &ChangeStoreyDriftXM, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    match base.buildings.get(payload.building_index).and_then(|b| b.storeys.get(payload.storey_index)) {
        Some(st) => vec![En1998Mutation::ChangeStoreyDriftXM(ChangeStoreyDriftXM { building_index: payload.building_index, storey_index: payload.storey_index, new_drift_x_m: st.drift_x_m })],
        None => Vec::new(),
    }
}
