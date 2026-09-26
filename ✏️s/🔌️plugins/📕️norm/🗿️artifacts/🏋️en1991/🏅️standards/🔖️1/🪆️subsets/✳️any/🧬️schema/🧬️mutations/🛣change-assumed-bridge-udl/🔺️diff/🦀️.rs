//! Diff for `change-assumed-bridge-udl`.
use super::ChangeAssumedBridgeUdl;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeAssumedBridgeUdl, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.assumed_bridge_udl == payload.new_assumed_bridge_udl {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { assumed_bridge_udl: Some(payload.new_assumed_bridge_udl), ..Default::default() })
}
