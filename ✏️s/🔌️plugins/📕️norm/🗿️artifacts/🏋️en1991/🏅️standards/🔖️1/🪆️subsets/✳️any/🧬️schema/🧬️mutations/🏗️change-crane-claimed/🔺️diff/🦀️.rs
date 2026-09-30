//! Diff for `change-crane-claimed`.
use super::ChangeCraneClaimed;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeCraneClaimed, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.crane_claimed == payload.new_crane_claimed {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { crane_claimed: Some(payload.new_crane_claimed), ..Default::default() })
}
