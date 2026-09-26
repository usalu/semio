//! Inverse for `change-storey-permanent-gk-n`.
use super::ChangeStoreyPermanentGkN;
use crate::{En1998Mutation, En1998Snapshot};

pub fn inverse(payload: &ChangeStoreyPermanentGkN, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    match base.buildings.get(payload.building_index).and_then(|b| b.storeys.get(payload.storey_index)) {
        Some(st) => vec![En1998Mutation::ChangeStoreyPermanentGkN(ChangeStoreyPermanentGkN {
            building_index: payload.building_index,
            storey_index: payload.storey_index,
            new_permanent_gk_n: st.permanent_gk_n,
        })],
        None => Vec::new(),
    }
}
