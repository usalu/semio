//! ↩️ Inverse for `ConnectSlots` — the `disconnect-slots` of the id it created.

use crate::artifacts::assembly::mutations::{disconnect_slots, AssemblyMutation};
use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;

pub fn inverse(payload: &super::mutation::ConnectSlots, _base: &AssemblySnapshot) -> Vec<AssemblyMutation> {
    vec![disconnect_slots(payload.edge.id.clone())]
}
