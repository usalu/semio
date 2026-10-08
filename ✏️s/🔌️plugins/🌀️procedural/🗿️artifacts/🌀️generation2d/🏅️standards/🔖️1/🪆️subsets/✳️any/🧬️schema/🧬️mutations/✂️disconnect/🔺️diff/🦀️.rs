//! 🔺️ Sparse diff builder for `DisconnectSynapse` — a real id-keyed removal from the fixture's
//! synapse collection helper (never a whole-snapshot capture).

use crate::standards::v1::subsets::any::schema::diff::{Generation2dDiff, Generation2dSynapsesDelta};
use crate::Generation2dSnapshot;

pub fn diff(payload: &super::DisconnectSynapse, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
    let Some(index) = base.host_snapshot.synapses.iter().position(|synapse| synapse.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Synapse \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(Generation2dDiff { synapses: Some(Generation2dSynapsesDelta::removal(&base.host_snapshot.synapses, index)), ..Default::default() })
}
