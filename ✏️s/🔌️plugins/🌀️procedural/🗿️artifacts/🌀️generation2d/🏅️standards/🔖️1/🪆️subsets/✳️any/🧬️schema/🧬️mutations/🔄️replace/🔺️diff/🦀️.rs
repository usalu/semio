//! 🔺️ Sparse diff for `ReplaceSynapse`, built directly from `(payload, base)`.
use super::ReplaceSynapse;
use crate::standards::v1::subsets::any::schema::diff::{Generation2dDiff, Generation2dSynapsePatchEntry, Generation2dSynapsesDelta};
use crate::Generation2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceSynapse, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
    if !base.host_snapshot.synapses.iter().any(|synapse| synapse.id == payload.synapse.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Synapse \"{}\" does not exist.", payload.synapse.id), [payload.synapse.id.clone()]);
    }
    protocol::MutationOutcome::new(Generation2dDiff { synapses: Some(Generation2dSynapsesDelta { patched: vec![Generation2dSynapsePatchEntry { id: payload.synapse.id.clone(), item: payload.synapse.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
