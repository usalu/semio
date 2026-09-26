//! Inverse for `change-assumed-bridge-udl`.
use super::ChangeAssumedBridgeUdl;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeAssumedBridgeUdl, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeAssumedBridgeUdl(ChangeAssumedBridgeUdl { new_assumed_bridge_udl: base.assumed_bridge_udl })]
}
