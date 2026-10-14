//! 🔺️ Sparse diff for `ReplaceSynapse`, built directly from `(payload, base)`.
use super::ReplaceSynapse;
use semio_s_artifact_procedural_2d_model::diff::Generation2dDiff;
use semio_s_artifact_procedural_2d_model::diff::Generation2dSynapseModification;
use semio_s_artifact_procedural_2d_model::diff::Generation2dSynapsesDelta;
use semio_s_artifact_procedural_2d_model::diff::Generation2dSynapsePatch;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceSynapse, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
    if !base.host_snapshot.synapses.iter().any(|synapse| synapse.id == payload.synapse.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Synapse \"{}\" does not exist.", payload.synapse.id), [payload.synapse.id.clone()]);
    }
    protocol::MutationOutcome::new(Generation2dDiff { synapses: Some(Generation2dSynapsesDelta { modified: vec![Generation2dSynapseModification { id: payload.synapse.id.clone(), patch: Generation2dSynapsePatch(payload.synapse.clone()) }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
