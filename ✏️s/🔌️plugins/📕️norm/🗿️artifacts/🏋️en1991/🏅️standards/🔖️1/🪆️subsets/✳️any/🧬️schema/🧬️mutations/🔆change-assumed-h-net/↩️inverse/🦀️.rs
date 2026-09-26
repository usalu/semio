//! Inverse for `change-assumed-h-net`.
use super::ChangeAssumedHNet;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeAssumedHNet, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeAssumedHNet(ChangeAssumedHNet { new_assumed_h_net: base.assumed_h_net })]
}
