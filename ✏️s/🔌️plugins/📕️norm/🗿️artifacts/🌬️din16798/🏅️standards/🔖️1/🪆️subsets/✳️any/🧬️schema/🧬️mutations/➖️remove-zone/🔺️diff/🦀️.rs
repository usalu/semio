//! ➖️ `remove-zone` diff — removes the row with that id; an id the document does not hold is a `mutation.target-missing`.

use super::RemoveZone;
use crate::diff::Din16798RowEdit as _;
use crate::diff::{Din16798Diff, Din16798ZoneEdit};
use crate::Din16798Snapshot;

pub fn diff(payload: &RemoveZone, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    let Some((index, row)) = base.zones.iter().enumerate().find(|(_, row)| row.id == payload.zone_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No zone has id '{}'.", payload.zone_id), [payload.zone_id.clone()]);
    };
    protocol::MutationOutcome::new(Din16798Diff { zones: vec![Din16798ZoneEdit::remove(index, row.id.clone())], ..Default::default() })
}
