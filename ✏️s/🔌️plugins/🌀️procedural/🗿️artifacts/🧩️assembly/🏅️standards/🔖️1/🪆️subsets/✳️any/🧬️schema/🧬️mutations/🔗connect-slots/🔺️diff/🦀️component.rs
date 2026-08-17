//! 🔺️ Sparse diff builder for `ConnectSlots` — a real id-keyed upsert into `edges`.

use crate::artifacts::assembly::diff::AssemblyDiff;
use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;

pub fn diff(payload: &super::mutation::ConnectSlots, base: &AssemblySnapshot) -> protocol::MutationOutcome<AssemblyDiff> {
    let edge = &payload.edge;
    if base.edges.iter().any(|existing| existing.id == edge.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("An edge with id \"{}\" already exists.", edge.id), [edge.id.clone()]);
    }
    if !base.slots.iter().any(|slot| slot.id == edge.from_slot_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Slot \"{}\" does not exist.", edge.from_slot_id), [edge.from_slot_id.clone()]);
    }
    if !base.slots.iter().any(|slot| slot.id == edge.to_slot_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Slot \"{}\" does not exist.", edge.to_slot_id), [edge.to_slot_id.clone()]);
    }
    if edge.from_slot_id == edge.to_slot_id {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Slot \"{}\" cannot connect to itself.", edge.from_slot_id), [edge.from_slot_id.clone()]);
    }
    if base.edges.iter().any(|existing| (existing.from_slot_id == edge.from_slot_id && existing.to_slot_id == edge.to_slot_id) || (existing.from_slot_id == edge.to_slot_id && existing.to_slot_id == edge.from_slot_id)) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("\"{}\" is already connected to \"{}\"; parallel edges are not allowed.", edge.from_slot_id, edge.to_slot_id));
    }
    protocol::MutationOutcome::new(AssemblyDiff { edges_upserted: vec![(payload.index, edge.clone())], ..Default::default() })
}
