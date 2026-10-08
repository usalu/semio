//! ➖️ `remove-zone` diff — removes the row with that id; an id the document does not hold is a `mutation.target-missing`.

use super::RemoveZone;
use crate::diff::{Din16798Diff, Din16798ZoneDelta};
use crate::Din16798Snapshot;

pub fn diff(payload: &RemoveZone, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    let Some(row) = base.zones.iter().find(|row| row.id == payload.zone_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No zone has id '{}'.", payload.zone_id), [payload.zone_id.clone()]);
    };
    protocol::MutationOutcome::new(Din16798Diff { zones: Din16798ZoneDelta::removal(&row.id), ..Default::default() })
}
