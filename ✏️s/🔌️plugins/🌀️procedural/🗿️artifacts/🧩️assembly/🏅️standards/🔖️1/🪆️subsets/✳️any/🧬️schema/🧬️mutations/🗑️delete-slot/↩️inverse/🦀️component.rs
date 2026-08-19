//! ↩️ Inverse for `DeleteSlot` — recreates the slot AND every incident edge the delete cascaded
//! away, all from a real BASE lookup (missing id ⇒ empty: no-op, nothing to undo).

use crate::artifacts::assembly::mutations::{connect_slots, create_slot, AssemblyMutation};
use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;

pub async fn inverse(payload: &super::mutation::DeleteSlot, base: &AssemblySnapshot) -> Vec<AssemblyMutation> {
    let Some(slot) = base.slots.iter().find(|slot| slot.id == payload.id) else {
        return Vec::new();
    };
    let index = base.slots.iter().position(|entry| entry.id == payload.id).unwrap_or(base.slots.len());
    let mut restore = vec![create_slot(index, slot.clone())];
    for (edge_index, edge) in base.edges.iter().enumerate() {
        if edge.from_slot_id == payload.id || edge.to_slot_id == payload.id {
            restore.push(connect_slots(edge_index, edge.clone()));
        }
    }
    restore
}
