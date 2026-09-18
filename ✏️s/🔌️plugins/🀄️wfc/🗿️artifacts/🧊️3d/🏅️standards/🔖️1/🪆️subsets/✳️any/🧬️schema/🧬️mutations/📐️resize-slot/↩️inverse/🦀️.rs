//! ↩️ Inverse for `ResizeSlot` — restores the slot's BASE extent (missing id ⇒ empty).

use crate::mutations::{resize_slot, Wfc3dMutation};
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn inverse(payload: &super::ResizeSlot, base: &Wfc3dSnapshot) -> Vec<Wfc3dMutation> {
    let Some(slot) = base.slots.iter().find(|slot| slot.id == payload.id) else {
        return Vec::new();
    };
    vec![resize_slot(slot.id.clone(), slot.width, slot.height, slot.depth)]
}
