//! ↩️ Inverse for `UnpinSlot` — re-pins the slot to the tile it named in BASE (no pin ⇒ empty).

use crate::mutations::{pin_slot, Wfc3dMutation};
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn inverse(payload: &super::UnpinSlot, base: &Wfc3dSnapshot) -> Vec<Wfc3dMutation> {
    let Some(slot) = base.slots.iter().find(|slot| slot.id == payload.id) else {
        return Vec::new();
    };
    match &slot.pinned_tile_id {
        Some(previous) => vec![pin_slot(slot.id.clone(), previous.clone())],
        None => Vec::new(),
    }
}
