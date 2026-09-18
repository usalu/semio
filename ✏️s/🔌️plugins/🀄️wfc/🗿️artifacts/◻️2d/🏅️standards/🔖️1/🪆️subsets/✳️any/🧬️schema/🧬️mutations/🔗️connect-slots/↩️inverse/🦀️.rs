//! ↩️ Inverse for `ConnectSlots` — built from a real BASE lookup, so a target the base never held
//! yields an empty inverse (nothing to undo) rather than a fabricated one.

use crate::mutations::{disconnect_slots, Wfc2dMutation};
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn inverse(payload: &super::ConnectSlots, _base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
    vec![disconnect_slots(payload.edge.id.clone())]
}
