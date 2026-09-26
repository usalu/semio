//! Diff for `change-assumed-h-net`.
use super::ChangeAssumedHNet;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeAssumedHNet, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.assumed_h_net == payload.new_assumed_h_net {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { assumed_h_net: Some(payload.new_assumed_h_net), ..Default::default() })
}
