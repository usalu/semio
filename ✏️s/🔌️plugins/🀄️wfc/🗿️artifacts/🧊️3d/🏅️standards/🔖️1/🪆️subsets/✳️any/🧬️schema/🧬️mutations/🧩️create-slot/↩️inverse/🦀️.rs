//! ↩️ Inverse for `CreateSlot` — the `delete-slot` of the id it created (the payload carries the id,
//! so no BASE lookup is needed to know what to undo).

use crate::mutations::{delete_slot, Wfc3dMutation};
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn inverse(payload: &super::CreateSlot, _base: &Wfc3dSnapshot) -> Vec<Wfc3dMutation> {
    vec![delete_slot(payload.slot.id.clone())]
}
