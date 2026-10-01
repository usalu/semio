//! 🔺️ `remove-vent-system` diff — removes the ventilation system with that id; an id the document does not hold is a
//! `mutation.target-missing`.

use super::RemoveVentSystem;
use crate::standards::v1::subsets::any::schema::diff::Din16798VentList;
use crate::{Din16798Diff, Din16798Snapshot};

pub fn diff(payload: &RemoveVentSystem, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !base.vent_systems.iter().any(|vent| vent.id == payload.vent_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No ventilation system has id '{}'.", payload.vent_id), [payload.vent_id.clone()]);
    }
    let vents = base.vent_systems.iter().filter(|vent| vent.id != payload.vent_id).cloned().collect();
    protocol::MutationOutcome::new(Din16798Diff { vent_systems: Some(Din16798VentList { values: vents }), ..Default::default() })
}
