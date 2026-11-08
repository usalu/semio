//! 🔺️ `insert-zone` sparse diff: one inserted row.

use crate::insert_zone::InsertZone;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;
use semio_s_artifact_norm_din18599_diff::{Din18599Diff, Din18599ZonesRows};

pub fn diff(payload: &InsertZone, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    if payload.index > base.zones.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Position {} is beyond the zones.", payload.index), [payload.index.to_string()]);
    }
    if base.zones.iter().any(|row| row.id == payload.zone.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Zone id {} already exists.", payload.zone.id), [payload.zone.id.clone()]);
    }
    protocol::MutationOutcome::new(Din18599Diff { zones: Some(Din18599ZonesRows::insertion(payload.index, payload.zone.clone())), ..Default::default() })
}
