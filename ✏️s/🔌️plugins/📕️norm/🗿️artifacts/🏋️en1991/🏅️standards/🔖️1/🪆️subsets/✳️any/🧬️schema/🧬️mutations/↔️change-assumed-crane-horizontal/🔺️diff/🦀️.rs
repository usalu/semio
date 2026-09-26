//! Diff for `change-assumed-crane-horizontal`.
use super::ChangeAssumedCraneHorizontal;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeAssumedCraneHorizontal, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.assumed_crane_horizontal == payload.new_assumed_crane_horizontal {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { assumed_crane_horizontal: Some(payload.new_assumed_crane_horizontal), ..Default::default() })
}
