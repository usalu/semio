//! Diff for `change-assumed-bridge-lm4`.
use super::ChangeAssumedBridgeLm4;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeAssumedBridgeLm4, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.assumed_bridge_lm4 == payload.new_assumed_bridge_lm4 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { assumed_bridge_lm4: Some(payload.new_assumed_bridge_lm4), ..Default::default() })
}
