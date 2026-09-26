//! Diff for `change-fire-opening-factor`.
use super::ChangeFireOpeningFactor;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeFireOpeningFactor, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.fire_opening_factor == payload.new_fire_opening_factor {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { fire_opening_factor: Some(payload.new_fire_opening_factor), ..Default::default() })
}
