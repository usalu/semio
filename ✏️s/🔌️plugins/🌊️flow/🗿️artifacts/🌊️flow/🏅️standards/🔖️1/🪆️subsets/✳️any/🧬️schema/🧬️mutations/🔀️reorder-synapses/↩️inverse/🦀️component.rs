//! ↩️ Undo mutation for `reorder-synapses`: reorder back to the base-state index.
use crate::artifacts::flow::schema::mutations::FlowMutation;
use crate::artifacts::flow::FlowSnapshot;

use super::mutation::ReorderSynapses;

pub fn inverse(payload: &ReorderSynapses, base: &FlowSnapshot) -> Vec<FlowMutation> {
    let Some(original_index) = base.synapses.iter().position(|synapse| synapse.id == payload.id) else {
        return Vec::new();
    };
    vec![FlowMutation::ReorderSynapses(ReorderSynapses { id: payload.id.clone(), to_index: original_index })]
}
