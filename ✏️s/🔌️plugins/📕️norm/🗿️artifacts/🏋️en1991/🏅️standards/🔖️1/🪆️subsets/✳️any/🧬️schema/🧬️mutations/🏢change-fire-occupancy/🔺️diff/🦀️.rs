//! Diff for `change-fire-occupancy`.
use super::ChangeFireOccupancy;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeFireOccupancy, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.fire_occupancy == payload.new_fire_occupancy {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { fire_occupancy: Some(payload.new_fire_occupancy.clone()), ..Default::default() })
}
