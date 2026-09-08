//! ↩️ `disconnect-synapse` inverse — reconstructs a `connect-synapse` from BASE state; an edge
//! already absent from `base` has nothing to undo.

use crate::mutations::connect_synapse::ConnectSynapse;
use crate::mutations::disconnect_synapse::DisconnectSynapse;
use crate::mutations::{synapse_index, Generation3dMutation};
use crate::Generation3dSnapshot;

/// ↩️ Missing id in `base` ⇒ `Vec::new()`.
pub fn inverse(payload: &DisconnectSynapse, base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
    match synapse_index(&base.fixture, &payload.id) {
        Some(index) => vec![Generation3dMutation::ConnectSynapse(ConnectSynapse { index, synapse: base.fixture.synapses[index].clone() })],
        None => Vec::new(),
    }
}
