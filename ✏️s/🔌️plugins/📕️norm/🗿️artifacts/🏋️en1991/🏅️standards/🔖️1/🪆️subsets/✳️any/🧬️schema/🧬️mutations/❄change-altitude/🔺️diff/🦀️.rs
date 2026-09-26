//! Diff for `change-altitude`.
use super::ChangeAltitude;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeAltitude, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.altitude == payload.new_altitude {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { altitude: Some(payload.new_altitude), ..Default::default() })
}
