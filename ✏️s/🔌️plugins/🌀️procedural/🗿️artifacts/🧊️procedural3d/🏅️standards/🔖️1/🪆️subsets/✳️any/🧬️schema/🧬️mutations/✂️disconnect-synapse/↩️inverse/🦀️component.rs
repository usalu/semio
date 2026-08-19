//! ↩️ `disconnect-synapse` inverse — reconstructs a `connect-synapse` from BASE state; an edge
//! already absent from `base` has nothing to undo.

use crate::artifacts::procedural3d::mutations::connect_synapse::mutation::ConnectSynapse;
use crate::artifacts::procedural3d::mutations::disconnect_synapse::mutation::DisconnectSynapse;
use crate::artifacts::procedural3d::mutations::{synapse_index, Procedural3dMutation};
use crate::artifacts::procedural3d::Procedural3dSnapshot;

/// ↩️ Missing id in `base` ⇒ `Vec::new()`.
pub async fn inverse(payload: &DisconnectSynapse, base: &Procedural3dSnapshot) -> Vec<Procedural3dMutation> {
    match synapse_index(&base.fixture, &payload.id) {
        Some(index) => vec![Procedural3dMutation::ConnectSynapse(ConnectSynapse { index, synapse: base.fixture.synapses[index].clone() })],
        None => Vec::new()}
}
