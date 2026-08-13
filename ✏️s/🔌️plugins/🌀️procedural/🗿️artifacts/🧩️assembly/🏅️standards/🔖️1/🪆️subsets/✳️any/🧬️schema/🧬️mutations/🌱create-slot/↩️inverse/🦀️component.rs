//! ↩️ Inverse for `CreateSlot` — the `delete-slot` of the id it created (the payload itself carries
//! the id, so no BASE lookup is needed to know what to undo).

use crate::artifacts::assembly::mutations::{delete_slot, AssemblyMutation};
use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;

pub fn inverse(payload: &super::mutation::CreateSlot, _base: &AssemblySnapshot) -> Vec<AssemblyMutation> {
    vec![delete_slot(payload.slot.id.clone())]
}
