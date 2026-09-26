//! Inverse for `change-assumed-bridge-tandem`.
use super::ChangeAssumedBridgeTandem;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeAssumedBridgeTandem, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeAssumedBridgeTandem(ChangeAssumedBridgeTandem { new_assumed_bridge_tandem: base.assumed_bridge_tandem })]
}
