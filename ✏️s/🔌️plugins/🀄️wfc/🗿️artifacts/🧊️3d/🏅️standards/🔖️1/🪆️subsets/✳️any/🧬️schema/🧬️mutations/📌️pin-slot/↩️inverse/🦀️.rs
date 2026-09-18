//! ↩️ Inverse for `PinSlot` — restores the slot's PRIOR pin, or unpins it when there was none.

use crate::mutations::{pin_slot, unpin_slot, Wfc3dMutation};
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn inverse(payload: &super::PinSlot, base: &Wfc3dSnapshot) -> Vec<Wfc3dMutation> {
    let Some(slot) = base.slots.iter().find(|slot| slot.id == payload.id) else {
        return Vec::new();
    };
    match &slot.pinned_tile_id {
        Some(previous) => vec![pin_slot(slot.id.clone(), previous.clone())],
        None => vec![unpin_slot(slot.id.clone())],
    }
}
