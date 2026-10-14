//! ↩️ `disconnect-synapse` inverse — reconstructs a `connect-synapse` from BASE state; an edge
//! already absent from `base` has nothing to undo.

use crate::connect_synapse::ConnectSynapse;
use crate::disconnect_synapse::DisconnectSynapse;
use semio_s_artifact_procedural_3d_model::synapse_index;

use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

/// ↩️ Missing id in `base` ⇒ `Vec::new()`.
pub fn inverse<Op: From<crate::connect_synapse::ConnectSynapse>>(payload: &DisconnectSynapse, base: &Generation3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    match synapse_index(&base.host_snapshot, &payload.id) {
        Some(index) => vec![Op::from(ConnectSynapse { index, synapse: base.host_snapshot.synapses[index].clone() })],
        None => Vec::new(),
    }

    })())
}
