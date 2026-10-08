//! 🆕️ `insert-vent-system` diff — inserts the row at its position; a position past the list's end inserts it last as a
//! `mutation.clamped` warning, and an id the document already holds is a `mutation.duplicate-id`.

use super::InsertVentSystem;
use crate::diff::{Din16798Diff, Din16798VentSystemDelta};
use crate::Din16798Snapshot;

pub fn diff(payload: &InsertVentSystem, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if base.vent_systems.iter().any(|existing| existing.id == payload.vent.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A ventilation system with id '{}' already exists.", payload.vent.id), [payload.vent.id.clone()]);
    }
    let index = payload.index.min(base.vent_systems.len());
    let outcome = protocol::MutationOutcome::new(Din16798Diff { vent_systems: Din16798VentSystemDelta::insertion(&base.vent_systems, index, payload.vent.clone()), ..Default::default() });
    if index == payload.index {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the ventilation system list; inserted at {index}.", payload.index))
}
