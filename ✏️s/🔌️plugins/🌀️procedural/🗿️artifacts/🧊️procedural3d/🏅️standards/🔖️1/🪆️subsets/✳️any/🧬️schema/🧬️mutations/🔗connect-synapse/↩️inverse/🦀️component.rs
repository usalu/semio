//! ↩️ `connect-synapse` inverse — undo of a connect is always a `disconnect-synapse` by id.

use crate::artifacts::procedural3d::mutations::connect_synapse::mutation::ConnectSynapse;
use crate::artifacts::procedural3d::mutations::remove_synapse::mutation::DisconnectSynapse;
use crate::artifacts::procedural3d::mutations::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;

/// ↩️ Undoing a connect is severing the same edge back out, by its own id.
pub fn inverse(payload: &ConnectSynapse, _base: &Procedural3dSnapshot) -> Vec<Procedural3dMutation> {
    vec![Procedural3dMutation::DisconnectSynapse(DisconnectSynapse { id: payload.synapse.id.clone() })]
}
