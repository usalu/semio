//! 🔺️ `insert-zone` diff — inserts the zone at its position; a position past the list's end inserts it last as a
//! `mutation.clamped` warning, and an id the document already holds is a `mutation.duplicate-id`.

use super::InsertZone;
use crate::standards::v1::subsets::any::schema::diff::Din16798ZoneList;
use crate::{Din16798Diff, Din16798Snapshot};

pub fn diff(payload: &InsertZone, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if base.zones.iter().any(|existing| existing.id == payload.zone.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A zone with id '{}' already exists.", payload.zone.id), [payload.zone.id.clone()]);
    }
    let mut values = base.zones.clone();
    let index = payload.index.min(values.len());
    values.insert(index, payload.zone.clone());
    let outcome = protocol::MutationOutcome::new(Din16798Diff { zones: Some(Din16798ZoneList { values }), ..Default::default() });
    if index == payload.index {
        return outcome;
    }
    outcome.warn("mutation.clamped", format!("Position {} is past the end of the zone list; inserted at {index}.", payload.index))
}
