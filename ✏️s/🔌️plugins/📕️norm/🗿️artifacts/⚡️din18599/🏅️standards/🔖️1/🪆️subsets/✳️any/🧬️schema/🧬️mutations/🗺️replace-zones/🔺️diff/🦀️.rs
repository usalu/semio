//! 🔺️ `replace-zones` sparse diff.

use crate::diff::Din18599Diff;
use crate::mutations::replace_zones::ReplaceZones;
use crate::Din18599Snapshot;

pub fn diff(payload: &ReplaceZones, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {

    if base.zones == payload.new_zones {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "zones already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff { zones: Some(crate::diff::Din18599ZoneList { values: payload.new_zones.clone() }), ..Default::default() })
}
