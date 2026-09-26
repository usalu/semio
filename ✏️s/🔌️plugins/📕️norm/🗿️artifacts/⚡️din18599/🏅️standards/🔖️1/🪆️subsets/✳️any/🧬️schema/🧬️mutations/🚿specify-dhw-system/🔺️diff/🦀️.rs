//! 🔺️ `specify-dhw-system` sparse diff.

use crate::diff::Din18599Diff;
use crate::mutations::specify_dhw_system::SpecifyDhwSystem;
use crate::Din18599Snapshot;

pub fn diff(payload: &SpecifyDhwSystem, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {

    if base.dhw == payload.new_dhw {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "dhw already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff { dhw: Some(payload.new_dhw.clone()), ..Default::default() })
}
