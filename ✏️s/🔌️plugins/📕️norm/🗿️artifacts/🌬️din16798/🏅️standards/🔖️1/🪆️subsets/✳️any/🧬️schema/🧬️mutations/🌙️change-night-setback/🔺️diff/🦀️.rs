//! 🔺️ `change-night-setback` diff.
use super::ChangeNightSetback;
use crate::{Din16798Diff, Din16798Snapshot};
pub fn diff(payload: &ChangeNightSetback, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if base.night_setback_k == payload.new_night_setback_k {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(Din16798Diff { night_setback_k: Some(payload.new_night_setback_k), ..Default::default() })
}
