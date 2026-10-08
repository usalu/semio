//! 🗑️ `remove-vent-system` diff — removes the row with that id; an id the document does not hold is a `mutation.target-missing`.

use super::RemoveVentSystem;
use crate::diff::Din16798RowEdit as _;
use crate::diff::{Din16798Diff, Din16798VentSystemEdit};
use crate::Din16798Snapshot;

pub fn diff(payload: &RemoveVentSystem, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    let Some((index, row)) = base.vent_systems.iter().enumerate().find(|(_, row)| row.id == payload.vent_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No ventilation system has id '{}'.", payload.vent_id), [payload.vent_id.clone()]);
    };
    protocol::MutationOutcome::new(Din16798Diff { vent_systems: vec![Din16798VentSystemEdit::remove(index, row.id.clone())], ..Default::default() })
}
