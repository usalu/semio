//! Diff for `change-assumed-crane-wheel`.
use super::ChangeAssumedCraneWheel;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeAssumedCraneWheel, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.assumed_crane_wheel == payload.new_assumed_crane_wheel {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { assumed_crane_wheel: Some(payload.new_assumed_crane_wheel), ..Default::default() })
}
