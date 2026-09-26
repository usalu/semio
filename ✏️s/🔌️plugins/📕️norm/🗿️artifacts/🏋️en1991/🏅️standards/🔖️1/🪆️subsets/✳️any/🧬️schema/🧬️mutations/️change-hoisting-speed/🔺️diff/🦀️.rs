//! Diff for `change-hoisting-speed`.
use super::ChangeHoistingSpeed;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeHoistingSpeed, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.hoisting_speed == payload.new_hoisting_speed {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { hoisting_speed: Some(payload.new_hoisting_speed), ..Default::default() })
}
