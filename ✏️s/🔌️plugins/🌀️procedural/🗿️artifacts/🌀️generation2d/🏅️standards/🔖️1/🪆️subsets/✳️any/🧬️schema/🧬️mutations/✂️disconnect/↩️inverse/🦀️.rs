//! ↩️ Inverse for `DisconnectSynapse` — reconnects the removed edge at its captured BASE index, or
//! a no-op (`Vec::new()`) when the id was already absent.

use crate::connect_synapse;

use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;

pub fn inverse<Op: From<crate::connect_synapse::ConnectSynapse>>(payload: &super::DisconnectSynapse, base: &Generation2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.host_snapshot.synapses.iter().position(|synapse| synapse.id == payload.id) {
        Some(index) => vec![connect_synapse(index, base.host_snapshot.synapses[index].clone())],
        None => Vec::new(),
    }

    })())
}
