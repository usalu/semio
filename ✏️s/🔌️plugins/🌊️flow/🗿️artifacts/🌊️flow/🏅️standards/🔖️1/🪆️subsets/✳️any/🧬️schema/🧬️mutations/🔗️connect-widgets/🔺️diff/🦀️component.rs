//! 🔺️ Sparse `FlowDiff` construction for `connect-widgets`.
use crate::artifacts::flow::schema::diff::text::{synapses_delta_from_collection_mutation, FlowDiff};
use crate::artifacts::flow::FlowSnapshot;
use flow::SynapseSpec;
use protocol::CollectionMutation;

use super::mutation::ConnectWidgets;

pub fn diff(payload: &ConnectWidgets, base: &FlowSnapshot) -> FlowDiff {
    let synapse = SynapseSpec { id: payload.id.clone(), from: payload.from.clone(), from_port: payload.from_port.clone(), to: payload.to.clone(), to_port: payload.to_port.clone() };
    let delta = synapses_delta_from_collection_mutation(&base.synapses, &CollectionMutation::Add { index: payload.index, item: synapse });
    FlowDiff { synapses: Some(delta), ..Default::default() }
}
