//! 🔺️ `disconnect-synapse` sparse diff construction.

use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
use crate::standards::v1::subsets::any::schema::diff::{Generation3dSynapsesDelta};
use crate::standards::v1::subsets::any::schema::mutations::disconnect_synapse::DisconnectSynapse;
use crate::standards::v1::subsets::any::schema::mutations::synapse_index;
use crate::Generation3dSnapshot;

/// 🏗️ Builds the sparse fixture delta severing one synapse edge by id.
pub fn diff(payload: &DisconnectSynapse, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    if synapse_index(&base.host_snapshot, &payload.id).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Synapse \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Generation3dDiff { synapses: Some(Generation3dSynapsesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
