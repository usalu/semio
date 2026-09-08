//! ↩️ `update-synapse` inverse — captures the pre-state edge from `base` and re-`update-synapse`s
//! back to it (self-inverse); missing target ⇒ nothing to undo.

use crate::standards::v1::subsets::any::schema::mutations::update_synapse::UpdateSynapse;
use crate::standards::v1::subsets::any::schema::mutations::{synapse_index, Generation3dMutation};
use crate::Generation3dSnapshot;

/// ↩️ Missing id in `base` ⇒ `Vec::new()`.
pub fn inverse(payload: &UpdateSynapse, base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
    match synapse_index(&base.fixture, &payload.synapse.id) {
        Some(index) => vec![Generation3dMutation::UpdateSynapse(UpdateSynapse { synapse: base.fixture.synapses[index].clone() })],
        None => Vec::new(),
    }
}
