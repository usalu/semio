//! Diff for `change-assumed-bridge-footway`.
use super::ChangeAssumedBridgeFootway;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeAssumedBridgeFootway, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.assumed_bridge_footway == payload.new_assumed_bridge_footway {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { assumed_bridge_footway: Some(payload.new_assumed_bridge_footway), ..Default::default() })
}
