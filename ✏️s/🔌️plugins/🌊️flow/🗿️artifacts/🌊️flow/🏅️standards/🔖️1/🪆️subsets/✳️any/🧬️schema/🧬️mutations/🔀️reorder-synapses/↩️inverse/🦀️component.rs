//! ↩️ Undo mutation for `reorder-synapses`: reorder back to the base-state index.
use crate::artifacts::flow::schema::mutations::FlowMutation;
use crate::artifacts::flow::{flow_working_scene, FlowSnapshot};

use super::mutation::ReorderSynapses;

pub fn inverse(payload: &ReorderSynapses, base: &FlowSnapshot) -> Vec<FlowMutation> {
    let scene = flow_working_scene(base);
    let Some(original_index) = scene.synapses.iter().position(|synapse| synapse.id == payload.id) else {
        return Vec::new();
    };
    vec![FlowMutation::ReorderSynapses(ReorderSynapses { id: payload.id.clone(), to_index: original_index })]
}
