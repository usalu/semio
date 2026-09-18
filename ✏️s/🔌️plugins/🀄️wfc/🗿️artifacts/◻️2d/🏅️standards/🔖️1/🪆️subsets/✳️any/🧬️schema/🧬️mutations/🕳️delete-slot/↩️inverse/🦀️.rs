//! ↩️ Inverse for `DeleteSlot` — built from a real BASE lookup, so a target the base never held
//! yields an empty inverse (nothing to undo) rather than a fabricated one.

use crate::mutations::{connect_slots, create_slot, Wfc2dMutation};
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn inverse(payload: &super::DeleteSlot, base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
    let Some(slot) = base.slots.iter().find(|slot| slot.id == payload.id) else {
        return Vec::new();
    };
    let mut restore = vec![create_slot(slot.clone())];
    for edge in &base.edges {
        if edge.from_slot_id == payload.id || edge.to_slot_id == payload.id {
            restore.push(connect_slots(edge.clone()));
        }
    }
    restore
}
