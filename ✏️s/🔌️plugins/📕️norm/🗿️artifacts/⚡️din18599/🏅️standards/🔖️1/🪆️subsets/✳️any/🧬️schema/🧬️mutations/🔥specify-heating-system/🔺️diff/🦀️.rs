//! 🔺️ `specify-heating-system` sparse diff.

use crate::diff::Din18599Diff;
use crate::mutations::specify_heating_system::SpecifyHeatingSystem;
use crate::Din18599Snapshot;

pub fn diff(payload: &SpecifyHeatingSystem, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {

    if base.heating == payload.new_heating {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "heating already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff { heating: Some(payload.new_heating.clone()), ..Default::default() })
}
