//! 🔺️ `update-lighting` sparse diff.

use crate::diff::Din18599Diff;
use crate::mutations::update_lighting::UpdateLighting;
use crate::Din18599Snapshot;

pub fn diff(payload: &UpdateLighting, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {

    if base.lighting == payload.new_lighting {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "lighting already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff { lighting: Some(payload.new_lighting.clone()), ..Default::default() })
}
