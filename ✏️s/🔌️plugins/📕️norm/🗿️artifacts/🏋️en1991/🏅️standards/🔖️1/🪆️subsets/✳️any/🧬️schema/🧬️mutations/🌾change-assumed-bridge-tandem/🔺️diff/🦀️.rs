//! Diff for `change-assumed-bridge-tandem`.
use super::ChangeAssumedBridgeTandem;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeAssumedBridgeTandem, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.assumed_bridge_tandem == payload.new_assumed_bridge_tandem {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { assumed_bridge_tandem: Some(payload.new_assumed_bridge_tandem), ..Default::default() })
}
