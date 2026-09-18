//! ↩️ Inverse for `DisconnectSlots` — recreates the edge at its BASE position (missing id ⇒ empty).

use crate::mutations::{connect_slots, Wfc3dMutation};
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn inverse(payload: &super::DisconnectSlots, base: &Wfc3dSnapshot) -> Vec<Wfc3dMutation> {
    let Some(index) = base.edges.iter().position(|edge| edge.id == payload.id) else {
        return Vec::new();
    };
    vec![connect_slots(index, base.edges[index].clone())]
}
