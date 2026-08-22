//! 🔺️ Sparse diff builder for `DeleteSlot` — removes the id from `slots` AND cascades to every
//! edge incident to it (real BASE lookup, not a whole-snapshot capture).

use crate::artifacts::assembly::diff::AssemblyDiff;
use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;

pub fn diff(payload: &super::mutation::DeleteSlot, base: &AssemblySnapshot) -> protocol::MutationOutcome<AssemblyDiff> {
    if !base.slots.iter().any(|slot| slot.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Slot \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let incident_edge_ids: Vec<String> = base.edges.iter().filter(|edge| edge.from_slot_id == payload.id || edge.to_slot_id == payload.id).map(|edge| edge.id.clone()).collect();
    let outcome = protocol::MutationOutcome::new(AssemblyDiff { slots_removed: vec![payload.id.clone()], edges_removed: incident_edge_ids.clone(), ..Default::default() });
    if incident_edge_ids.is_empty() {
        outcome
    } else {
        outcome.info("mutation.cascade", format!("Deleting slot \"{}\" also removed {} connected edge(s): {}.", payload.id, incident_edge_ids.len(), incident_edge_ids.join(", ")))
    }
}
