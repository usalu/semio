//! 🔺️ `connect-synapse` sparse diff construction.

use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
use crate::standards::v1::subsets::any::schema::diff::{Generation3dSynapsesDelta};
use crate::standards::v1::subsets::any::schema::mutations::connect_synapse::ConnectSynapse;
use crate::standards::v1::subsets::any::schema::mutations::{synapse_index,widget_index};

use crate::Generation3dSnapshot;

/// 🏗️ Builds the sparse fixture delta for one new synapse edge.
pub fn diff(payload: &ConnectSynapse, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    let id = &payload.synapse.id;
    if synapse_index(&base.host_snapshot, id).is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A synapse with id \"{id}\" already exists."), [id.clone()]);
    }
    if widget_index(&base.host_snapshot, &payload.synapse.from).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Synapse endpoint widget \"{}\" does not exist.", payload.synapse.from), [payload.synapse.from.clone()]);
    }
    if widget_index(&base.host_snapshot, &payload.synapse.to).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Synapse endpoint widget \"{}\" does not exist.", payload.synapse.to), [payload.synapse.to.clone()]);
    }
    let at = payload.index.min(base.host_snapshot.synapses.len());
    protocol::MutationOutcome::new(Generation3dDiff { synapses: Some(Generation3dSynapsesDelta::insertion(at, payload.synapse.clone())), ..Default::default() })
}
