//! ↩️ Inverse for `ConnectSlots` — the `disconnect-slots` of the id it created.

use crate::mutations::{disconnect_slots, Wfc3dMutation};
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn inverse(payload: &super::ConnectSlots, _base: &Wfc3dSnapshot) -> Vec<Wfc3dMutation> {
    vec![disconnect_slots(payload.edge.id.clone())]
}
