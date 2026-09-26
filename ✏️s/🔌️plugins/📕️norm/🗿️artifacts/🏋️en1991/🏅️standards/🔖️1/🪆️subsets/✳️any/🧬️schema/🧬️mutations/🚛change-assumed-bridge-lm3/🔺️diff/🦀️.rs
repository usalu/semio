//! Diff for `change-assumed-bridge-lm3`.
use super::ChangeAssumedBridgeLm3;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeAssumedBridgeLm3, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.assumed_bridge_lm3 == payload.new_assumed_bridge_lm3 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { assumed_bridge_lm3: Some(payload.new_assumed_bridge_lm3), ..Default::default() })
}
