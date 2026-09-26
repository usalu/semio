//! Diff for `change-assumed-bridge-lm2`.
use super::ChangeAssumedBridgeLm2;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeAssumedBridgeLm2, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.assumed_bridge_lm2 == payload.new_assumed_bridge_lm2 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { assumed_bridge_lm2: Some(payload.new_assumed_bridge_lm2), ..Default::default() })
}
