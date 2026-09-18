//! ↩️ Inverse for `DisconnectSlots` — built from a real BASE lookup, so a target the base never held
//! yields an empty inverse (nothing to undo) rather than a fabricated one.

use crate::mutations::{connect_slots, Wfc2dMutation};
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn inverse(payload: &super::DisconnectSlots, base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
    let Some(edge) = base.edges.iter().find(|edge| edge.id == payload.id) else {
        return Vec::new();
    };
    vec![connect_slots(edge.clone())]
}
