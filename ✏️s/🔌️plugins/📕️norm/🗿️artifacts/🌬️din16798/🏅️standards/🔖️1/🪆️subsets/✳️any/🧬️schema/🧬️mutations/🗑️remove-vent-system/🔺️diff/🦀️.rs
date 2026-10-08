//! 🗑️ `remove-vent-system` diff — removes the row with that id; an id the document does not hold is a `mutation.target-missing`.

use super::RemoveVentSystem;
use crate::diff::{Din16798Diff, Din16798VentSystemDelta};
use crate::Din16798Snapshot;

pub fn diff(payload: &RemoveVentSystem, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    let Some(row) = base.vent_systems.iter().find(|row| row.id == payload.vent_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No ventilation system has id '{}'.", payload.vent_id), [payload.vent_id.clone()]);
    };
    protocol::MutationOutcome::new(Din16798Diff { vent_systems: Din16798VentSystemDelta::removal(&row.id), ..Default::default() })
}
