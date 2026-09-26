//! Inverse for `change-bridge-lane`.
use super::ChangeBridgeLane;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeBridgeLane, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeBridgeLane(ChangeBridgeLane { new_bridge_lane: base.bridge_lane })]
}
