//! 🔺️ Sparse `FlowDiff` construction for `reorder-synapses` — recomputes the synapse id order from
//! `base` directly (never a whole-snapshot capture).
use crate::artifacts::flow::schema::diff::text::{FlowDiff, FlowSynapsesDelta};
use crate::artifacts::flow::FlowSnapshot;

use super::mutation::ReorderSynapses;

pub fn diff(payload: &ReorderSynapses, base: &FlowSnapshot) -> FlowDiff {
    let mut ids: Vec<String> = base.synapses.iter().map(|synapse| synapse.id.clone()).collect();
    if let Some(from) = ids.iter().position(|id| id == &payload.id) {
        let item = ids.remove(from);
        let to = payload.to_index.min(ids.len());
        ids.insert(to, item);
    }
    FlowDiff { synapses: Some(FlowSynapsesDelta { reordered: Some(ids), ..Default::default() }), ..Default::default() }
}
