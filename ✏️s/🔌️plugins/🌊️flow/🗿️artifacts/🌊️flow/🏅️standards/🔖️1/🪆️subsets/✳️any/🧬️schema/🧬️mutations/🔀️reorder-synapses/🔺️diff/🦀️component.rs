//! 🔺️ Sparse `FlowDiff` construction for `reorder-synapses`.
use crate::artifacts::flow::schema::diff::text::{synapses_delta_from_collection_mutation, FlowDiff};
use crate::artifacts::flow::FlowSnapshot;
use protocol::CollectionMutation;

use super::mutation::ReorderSynapses;

pub fn diff(payload: &ReorderSynapses, base: &FlowSnapshot) -> FlowDiff {
    let delta = synapses_delta_from_collection_mutation(&base.synapses, &CollectionMutation::Move { id: payload.id.clone(), to_index: payload.to_index });
    FlowDiff { synapses: Some(delta), ..Default::default() }
}
