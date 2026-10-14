//! ↩️ `connect-synapse` inverse — undo of a connect is always a `disconnect-synapse` by id.

use crate::connect_synapse::ConnectSynapse;
use crate::disconnect_synapse::DisconnectSynapse;
use crate::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

/// ↩️ Undoing a connect is severing the same edge back out, by its own id.
pub fn inverse(payload: &ConnectSynapse, _base: &Generation3dSnapshot) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Generation3dMutation::DisconnectSynapse(DisconnectSynapse { id: payload.synapse.id.clone() })]

    })())
}
