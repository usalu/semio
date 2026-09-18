//! ↩️ Inverse for `UnpinSlot` — built from a real BASE lookup, so a target the base never held
//! yields an empty inverse (nothing to undo) rather than a fabricated one.

use crate::mutations::{pin_slot, Wfc2dMutation};
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn inverse(payload: &super::UnpinSlot, base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
    let Some(slot) = base.slots.iter().find(|slot| slot.id == payload.id) else {
        return Vec::new();
    };
    match &slot.pinned_tile_id {
        Some(previous) => vec![pin_slot(payload.id.clone(), previous.clone())],
        None => Vec::new(),
    }
}
