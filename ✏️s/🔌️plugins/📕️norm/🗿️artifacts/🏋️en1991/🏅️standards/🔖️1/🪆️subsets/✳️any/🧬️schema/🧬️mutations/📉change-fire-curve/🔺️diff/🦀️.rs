//! Diff for `change-fire-curve`.
use super::ChangeFireCurve;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeFireCurve, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.fire_curve == payload.new_fire_curve {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { fire_curve: Some(payload.new_fire_curve), ..Default::default() })
}
