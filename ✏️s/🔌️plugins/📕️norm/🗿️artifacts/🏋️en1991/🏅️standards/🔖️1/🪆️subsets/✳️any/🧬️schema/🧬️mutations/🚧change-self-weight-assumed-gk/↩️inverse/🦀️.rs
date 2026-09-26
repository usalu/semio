//! Inverse for `change-self-weight-assumed-gk`.
use super::ChangeSelfWeightAssumedGk;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(payload: &ChangeSelfWeightAssumedGk, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    if payload.index >= base.self_weight_elements.len() { return Vec::new(); }
    vec![En1991Mutation::ChangeSelfWeightAssumedGk(ChangeSelfWeightAssumedGk { index: payload.index, new_assumed_gk: base.self_weight_elements[payload.index].assumed_gk })]
}
