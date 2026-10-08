//! ➕️ `insert-zone` diff — inserts the row at its position; a position past the list's end inserts it last as a
//! `mutation.clamped` warning, and an id the document already holds is a `mutation.duplicate-id`.

use super::InsertZone;
use crate::diff::{Din4108Diff, Din4108ZoneDelta};
use crate::Din4108Snapshot;

pub fn diff(payload: &InsertZone, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if base.zones.iter().any(|existing| existing.id == payload.zone.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A zone with id '{}' already exists.", payload.zone.id), [payload.zone.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.zones.len());
    let outcome = protocol::MutationOutcome::new(Din4108Diff { zones: Din4108ZoneDelta::insertion(index, payload.zone.clone()), ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the zone list; inserted at {index}.", payload.index.unwrap_or(index)))
}
