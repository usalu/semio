//! ↩️ `update-synapse` inverse — captures the pre-state edge from `base` and re-`update-synapse`s
//! back to it (self-inverse); missing target ⇒ nothing to undo.

use crate::artifacts::procedural3d::mutations::update_synapse::UpdateSynapse;
use crate::artifacts::procedural3d::mutations::{synapse_index, Procedural3dMutation};
use crate::artifacts::procedural3d::Procedural3dSnapshot;

/// ↩️ Missing id in `base` ⇒ `Vec::new()`.
pub fn inverse(payload: &UpdateSynapse, base: &Procedural3dSnapshot) -> Vec<Procedural3dMutation> {
    match synapse_index(&base.fixture, &payload.synapse.id) {
        Some(index) => vec![Procedural3dMutation::UpdateSynapse(UpdateSynapse { synapse: base.fixture.synapses[index].clone() })],
        None => Vec::new(),
    }
}
