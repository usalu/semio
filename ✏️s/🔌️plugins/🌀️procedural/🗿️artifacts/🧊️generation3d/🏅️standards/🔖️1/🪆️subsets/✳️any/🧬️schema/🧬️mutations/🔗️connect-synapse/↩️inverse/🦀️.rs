//! ↩️ `connect-synapse` inverse — undo of a connect is always a `disconnect-synapse` by id.

use crate::connect_synapse::ConnectSynapse;
use crate::disconnect_synapse::DisconnectSynapse;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

/// ↩️ Undoing a connect is severing the same edge back out, by its own id.
pub fn inverse<Op: From<crate::disconnect_synapse::DisconnectSynapse>>(payload: &ConnectSynapse, _base: &Generation3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Op::from(DisconnectSynapse { id: payload.synapse.id.clone() })]

    })())
}
