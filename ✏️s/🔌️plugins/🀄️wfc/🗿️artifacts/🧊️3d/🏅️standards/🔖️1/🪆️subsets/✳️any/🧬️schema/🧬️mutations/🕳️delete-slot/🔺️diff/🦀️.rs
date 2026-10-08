//! 🔺️ Sparse diff builder for `DeleteSlot` — removes the id from `slots` AND cascades to every edge
//! incident to it (real BASE lookup, not a whole-snapshot capture).

use crate::diff::{Wfc3dDiff, Wfc3dEdgesDelta, Wfc3dSlotsDelta};
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn diff(payload: &super::DeleteSlot, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
    if !base.slots.iter().any(|slot| slot.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Slot \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let incident_edge_ids: Vec<String> = base.edges.iter().filter(|edge| edge.from_slot_id == payload.id || edge.to_slot_id == payload.id).map(|edge| edge.id.clone()).collect();
    let outcome = protocol::MutationOutcome::new(Wfc3dDiff { slots: Wfc3dSlotsDelta::removal(&base.slots, base.slots.iter().position(|row| protocol::list_delta::Keyed::key(row) == payload.id.clone()).unwrap_or(usize::MAX)), edges: Wfc3dEdgesDelta::removals(&base.edges, &base.edges.iter().enumerate().filter(|(_, row)| incident_edge_ids.clone().contains(&protocol::list_delta::Keyed::key(*row))).map(|(index, _)| index).collect::<Vec<_>>()), ..Default::default() });
    if incident_edge_ids.is_empty() {
        outcome
    } else {
        outcome.info("mutation.cascade", format!("Deleting slot \"{}\" also removed {} adjacency edge(s): {}.", payload.id, incident_edge_ids.len(), incident_edge_ids.join(", ")))
    }
}
