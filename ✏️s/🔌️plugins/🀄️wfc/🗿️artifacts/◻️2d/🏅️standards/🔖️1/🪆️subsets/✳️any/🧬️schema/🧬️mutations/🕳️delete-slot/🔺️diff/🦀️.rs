//! 🔺️ Sparse diff builder for `DeleteSlot` — a real id-keyed delta, never a whole-snapshot capture.

use crate::diff::{Wfc2dDiff, Wfc2dEdgesDelta, Wfc2dSlotsDelta};
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn diff(payload: &super::DeleteSlot, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
    if !base.slots.iter().any(|slot| slot.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Slot \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let incident: Vec<String> = base.edges.iter().filter(|edge| edge.from_slot_id == payload.id || edge.to_slot_id == payload.id).map(|edge| edge.id.clone()).collect();
    let outcome = protocol::MutationOutcome::new(Wfc2dDiff { slots: Wfc2dSlotsDelta::removal(&base.slots, base.slots.iter().position(|row| protocol::list_delta::Keyed::key(row) == payload.id.clone()).unwrap_or(usize::MAX)), edges: Wfc2dEdgesDelta::removals(&base.edges, &base.edges.iter().enumerate().filter(|(_, row)| incident.clone().contains(&protocol::list_delta::Keyed::key(*row))).map(|(index, _)| index).collect::<Vec<_>>()), ..Default::default() });
    if incident.is_empty() {
        outcome
    } else {
        outcome.info("mutation.cascade", format!("Deleting slot \"{}\" also removed {} connected edge(s): {}.", payload.id, incident.len(), incident.join(", ")))
    }
}
