//! Diff for `change-fire-duration`.
use super::ChangeFireDuration;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeFireDuration, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.fire_duration == payload.new_fire_duration {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { fire_duration: Some(payload.new_fire_duration), ..Default::default() })
}
