//! 🔺️ Sparse `FlowDiff` construction for `disconnect-widgets`.
use crate::artifacts::flow::schema::diff::text::{synapses_delta_from_collection_mutation, FlowDiff};
use crate::artifacts::flow::FlowSnapshot;
use protocol::CollectionMutation;

use super::mutation::DisconnectWidgets;

pub fn diff(payload: &DisconnectWidgets, base: &FlowSnapshot) -> FlowDiff {
    let delta = synapses_delta_from_collection_mutation(&base.synapses, &CollectionMutation::Remove { id: payload.id.clone() });
    FlowDiff { synapses: Some(delta), ..Default::default() }
}
