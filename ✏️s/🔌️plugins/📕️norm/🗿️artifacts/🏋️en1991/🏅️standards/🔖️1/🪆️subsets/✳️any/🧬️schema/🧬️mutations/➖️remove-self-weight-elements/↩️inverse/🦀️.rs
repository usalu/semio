//! Inverse for `remove-self-weight-elements`.
use super::RemoveSelfWeightElements;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(payload: &RemoveSelfWeightElements, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    if payload.index >= base.self_weight_elements.len() { return Vec::new(); }
    let item = base.self_weight_elements[payload.index].clone();
    vec![En1991Mutation::InsertSelfWeightElements(crate::mutations::insert_self_weight_elements::InsertSelfWeightElements { index: payload.index, item })]
}
