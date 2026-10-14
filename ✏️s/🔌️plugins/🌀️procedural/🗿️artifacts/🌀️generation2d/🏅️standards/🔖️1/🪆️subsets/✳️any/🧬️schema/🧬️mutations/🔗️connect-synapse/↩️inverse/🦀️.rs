//! ↩️ Inverse for `ConnectSynapse` — the `disconnect-synapse` of the id it created (the payload
//! itself carries the id, so no BASE lookup is needed to know what to undo).

use crate::disconnect_synapse;

use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;

pub fn inverse<Op: From<crate::disconnect_synapse::DisconnectSynapse>>(payload: &super::ConnectSynapse, _base: &Generation2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![disconnect_synapse(payload.synapse.id.clone())]

    })())
}
