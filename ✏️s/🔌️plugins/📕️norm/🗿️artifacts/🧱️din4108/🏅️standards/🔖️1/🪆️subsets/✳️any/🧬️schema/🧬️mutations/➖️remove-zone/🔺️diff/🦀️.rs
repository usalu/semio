//! ➖️ `remove-zone` diff — removes the row at the index; an index past the list's end is a `mutation.invariant`.

use super::RemoveZone;
use crate::diff::{Din4108Diff, Din4108ZoneDelta};
use crate::Din4108Snapshot;

pub fn diff(payload: &RemoveZone, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    let Some(row) = base.zones.get(payload.index) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "zone index out of range", Vec::<String>::new());
    };
    protocol::MutationOutcome::new(Din4108Diff { zones: Din4108ZoneDelta::removal(&row.id), ..Default::default() })
}
