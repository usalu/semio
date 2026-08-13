//! 🔺️ Sparse diff builder for `DeleteSlot` — removes the id from `slots` AND cascades to every
//! edge incident to it (real BASE lookup, not a whole-snapshot capture).

use crate::artifacts::assembly::diff::AssemblyDiff;
use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;

pub fn diff(payload: &super::mutation::DeleteSlot, base: &AssemblySnapshot) -> AssemblyDiff {
    let incident_edge_ids: Vec<String> = base.edges.iter().filter(|edge| edge.from_slot_id == payload.id || edge.to_slot_id == payload.id).map(|edge| edge.id.clone()).collect();
    AssemblyDiff { slots_removed: vec![payload.id.clone()], edges_removed: incident_edge_ids, ..Default::default() }
}
