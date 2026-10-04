//! 🔺️ Sparse diff builder for `ConnectSlots` — a real id-keyed upsert into `edges`. Two slots may
//! carry several edges as long as each states a DIFFERENT relation; a duplicate relation is a no-op.

use crate::diff::Wfc3dDiff;
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn diff(payload: &super::ConnectSlots, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
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
    if base
        .edges
        .iter()
        .any(|existing| existing.relation == edge.relation && ((existing.from_slot_id == edge.from_slot_id && existing.to_slot_id == edge.to_slot_id) || (existing.from_slot_id == edge.to_slot_id && existing.to_slot_id == edge.from_slot_id)))
    {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("\"{}\" already relates to \"{}\" as \"{}\".", edge.from_slot_id, edge.to_slot_id, edge.relation));
    }
    protocol::MutationOutcome::new(Wfc3dDiff { edges_upserted: vec![(payload.index, edge.clone())], ..Default::default() })
}
