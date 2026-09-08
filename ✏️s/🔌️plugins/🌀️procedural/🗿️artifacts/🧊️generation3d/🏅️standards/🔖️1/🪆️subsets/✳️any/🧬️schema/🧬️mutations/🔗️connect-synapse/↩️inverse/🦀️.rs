//! ↩️ `connect-synapse` inverse — undo of a connect is always a `disconnect-synapse` by id.

use crate::standards::v1::subsets::any::schema::mutations::connect_synapse::ConnectSynapse;
use crate::standards::v1::subsets::any::schema::mutations::disconnect_synapse::DisconnectSynapse;
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;

/// ↩️ Undoing a connect is severing the same edge back out, by its own id.
pub fn inverse(payload: &ConnectSynapse, _base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
    vec![Generation3dMutation::DisconnectSynapse(DisconnectSynapse { id: payload.synapse.id.clone() })]
}
