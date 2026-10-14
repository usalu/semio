//! 🔺️ `replace-zones` sparse diff.

use crate::replace_zones::ReplaceZones;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;
use semio_s_artifact_norm_din18599_diff::{Din18599Diff, Din18599ZonesRows};

pub fn diff(payload: &ReplaceZones, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    if base.zones == payload.new_zones {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "zones already has this value.");
    }
    if let Some((_, row)) = payload.new_zones.iter().enumerate().find(|(at, row)| payload.new_zones[..*at].iter().any(|earlier| earlier.id == row.id)) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Zone id {} appears twice.", row.id), [row.id.clone()]);
    }
    protocol::MutationOutcome::new(Din18599Diff { zones: Some(Din18599ZonesRows::setting(&base.zones, &payload.new_zones)), ..Default::default() })
}
