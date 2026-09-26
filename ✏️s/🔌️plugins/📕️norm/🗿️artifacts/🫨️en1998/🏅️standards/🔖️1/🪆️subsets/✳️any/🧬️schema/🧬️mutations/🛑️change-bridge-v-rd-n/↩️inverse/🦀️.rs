//! Inverse for `change-bridge-v-rd-n`.
use super::ChangeBridgeVRdN;
use crate::{En1998Mutation, En1998Snapshot};

pub fn inverse(payload: &ChangeBridgeVRdN, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    match base.bridges.get(payload.index) {
        Some(b) => vec![En1998Mutation::ChangeBridgeVRdN(ChangeBridgeVRdN { index: payload.index, new_v_rd_n: b.v_rd_n })],
        None => Vec::new(),
    }
}
