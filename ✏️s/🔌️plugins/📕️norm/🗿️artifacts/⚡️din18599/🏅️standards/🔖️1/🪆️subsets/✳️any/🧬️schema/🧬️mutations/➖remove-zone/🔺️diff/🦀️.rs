//! 🔺️ `remove-zone` sparse diff: one removed row.

use crate::remove_zone::RemoveZone;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;
use semio_s_artifact_norm_din18599_diff::{Din18599Diff, Din18599ZonesRows};

pub fn diff(payload: &RemoveZone, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    let Some(index) = base.zones.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(Din18599Diff { zones: Some(Din18599ZonesRows::removal(&base.zones, index)), ..Default::default() })
}
