//! 🔺️ `update-synapse` sparse diff construction.

use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use semio_s_artifact_procedural_3d_model::diff::Generation3dSynapseModification;
use semio_s_artifact_procedural_3d_model::diff::Generation3dSynapsesDelta;
use semio_s_artifact_procedural_3d_model::diff::Generation3dSynapsePatch;
use crate::synapse_index;
use crate::update_synapse::UpdateSynapse;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

/// 🏗️ Builds the sparse fixture delta replacing one existing synapse's ports. The index is
/// irrelevant here — the synapses delta resolves an existing entry by id first.
pub fn diff(payload: &UpdateSynapse, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    let id = &payload.synapse.id;
    let Some(index) = synapse_index(&base.host_snapshot, id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Synapse \"{id}\" does not exist."), [id.clone()]);
    };
    if base.host_snapshot.synapses[index] == payload.synapse {
        return protocol::MutationOutcome::new(Generation3dDiff::default()).warning("mutation.no-op", format!("Synapse \"{id}\" is already in the requested state."));
    }
    protocol::MutationOutcome::new(Generation3dDiff { synapses: Some(Generation3dSynapsesDelta { modified: vec![Generation3dSynapseModification { id: id.clone(), patch: Generation3dSynapsePatch(payload.synapse.clone()) }], ..Default::default() }), ..Default::default() })
}
