//! ↩️ Inverse for `MoveSlot` — moves the slot back to its BASE position (missing id ⇒ empty).

use crate::mutations::{move_slot, Wfc3dMutation};
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn inverse(payload: &super::MoveSlot, base: &Wfc3dSnapshot) -> Vec<Wfc3dMutation> {
    let Some(slot) = base.slots.iter().find(|slot| slot.id == payload.id) else {
        return Vec::new();
    };
    vec![move_slot(slot.id.clone(), slot.x, slot.y, slot.z)]
}
