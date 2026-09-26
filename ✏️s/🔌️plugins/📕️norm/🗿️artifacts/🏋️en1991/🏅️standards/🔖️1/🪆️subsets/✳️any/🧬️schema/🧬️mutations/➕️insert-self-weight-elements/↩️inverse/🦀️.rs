//! Inverse for `insert-self-weight-elements`.
use super::InsertSelfWeightElements;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(payload: &InsertSelfWeightElements, _base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::RemoveSelfWeightElements(crate::mutations::remove_self_weight_elements::RemoveSelfWeightElements { index: payload.index })]
}
