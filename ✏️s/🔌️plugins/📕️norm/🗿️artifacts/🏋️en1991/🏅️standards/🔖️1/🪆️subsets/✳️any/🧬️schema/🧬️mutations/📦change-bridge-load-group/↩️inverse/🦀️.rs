//! Inverse for `change-bridge-load-group`.
use super::ChangeBridgeLoadGroup;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeBridgeLoadGroup, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeBridgeLoadGroup(ChangeBridgeLoadGroup { new_bridge_load_group: base.bridge_load_group.clone() })]
}
