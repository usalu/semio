//! 🔺️ `disconnect-synapse` sparse diff construction.

use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use semio_s_artifact_procedural_3d_model::diff::Generation3dSynapsesDelta;
use crate::disconnect_synapse::DisconnectSynapse;
use semio_s_artifact_procedural_3d_model::synapse_index;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

/// 🏗️ Builds the sparse fixture delta severing one synapse edge by id.
pub fn diff(payload: &DisconnectSynapse, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    let Some(index) = synapse_index(&base.host_snapshot, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Synapse \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(Generation3dDiff { synapses: Some(Generation3dSynapsesDelta::removal(&base.host_snapshot.synapses, index)), ..Default::default() })
}
