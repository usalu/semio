//! 🔺️ `update-cooling` sparse diff.

use crate::diff::Din18599Diff;
use crate::mutations::update_cooling::UpdateCooling;
use crate::Din18599Snapshot;

pub fn diff(payload: &UpdateCooling, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {

    if base.cooling == payload.new_cooling {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "cooling already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff { cooling: Some(payload.new_cooling.clone()), ..Default::default() })
}
