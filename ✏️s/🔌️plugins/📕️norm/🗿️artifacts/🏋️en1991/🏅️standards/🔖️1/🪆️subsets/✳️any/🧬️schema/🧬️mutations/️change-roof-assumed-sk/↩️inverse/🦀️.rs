//! Inverse for `change-roof-assumed-sk`.
use super::ChangeRoofAssumedSk;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(payload: &ChangeRoofAssumedSk, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    if payload.index >= base.roofs.len() { return Vec::new(); }
    vec![En1991Mutation::ChangeRoofAssumedSk(ChangeRoofAssumedSk { index: payload.index, new_assumed_sk: base.roofs[payload.index].assumed_sk })]
}
