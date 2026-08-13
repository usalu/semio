//! ↩️ Inverse for `DisconnectSlots` — recreates the edge from a real BASE lookup (missing id ⇒
//! empty: no-op, nothing to undo).

use crate::artifacts::assembly::mutations::{connect_slots, AssemblyMutation};
use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;

pub fn inverse(payload: &super::mutation::DisconnectSlots, base: &AssemblySnapshot) -> Vec<AssemblyMutation> {
    let Some(index) = base.edges.iter().position(|edge| edge.id == payload.id) else {
        return Vec::new();
    };
    vec![connect_slots(index, base.edges[index].clone())]
}
