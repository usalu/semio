//! Inverse for `change-assumed-bridge-lm3`.
use super::ChangeAssumedBridgeLm3;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeAssumedBridgeLm3, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeAssumedBridgeLm3(ChangeAssumedBridgeLm3 { new_assumed_bridge_lm3: base.assumed_bridge_lm3 })]
}
