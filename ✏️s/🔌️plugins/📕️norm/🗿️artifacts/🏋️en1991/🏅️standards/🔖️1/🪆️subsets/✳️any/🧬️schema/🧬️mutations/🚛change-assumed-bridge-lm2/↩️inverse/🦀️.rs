//! Inverse for `change-assumed-bridge-lm2`.
use super::ChangeAssumedBridgeLm2;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeAssumedBridgeLm2, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeAssumedBridgeLm2(ChangeAssumedBridgeLm2 { new_assumed_bridge_lm2: base.assumed_bridge_lm2 })]
}
