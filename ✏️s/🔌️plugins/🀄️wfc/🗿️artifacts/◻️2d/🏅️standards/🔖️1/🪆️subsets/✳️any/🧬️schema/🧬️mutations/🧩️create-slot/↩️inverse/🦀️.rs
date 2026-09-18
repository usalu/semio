//! ↩️ Inverse for `CreateSlot` — built from a real BASE lookup, so a target the base never held
//! yields an empty inverse (nothing to undo) rather than a fabricated one.

use crate::mutations::{delete_slot, Wfc2dMutation};
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn inverse(payload: &super::CreateSlot, _base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
    vec![delete_slot(payload.slot.id.clone())]
}
