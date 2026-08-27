//! 🔺️ Sparse `FlowDiff` construction for `reorder-synapses` — recomputes the synapse order from the
//! current working scene directly (never a whole-snapshot capture).
use crate::artifacts::flow::schema::diff::text::{diff_replace_content, FlowDiff};
use crate::artifacts::flow::{flow_working_scene, FlowSnapshot};

use super::mutation::ReorderSynapses;

pub fn diff(payload: &ReorderSynapses, base: &FlowSnapshot) -> protocol::MutationOutcome<FlowDiff> {
    let mut scene = flow_working_scene(base);
    let Some(from) = scene.synapses.iter().position(|synapse| synapse.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Synapse \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let to = payload.to_index.min(scene.synapses.len().saturating_sub(1));
    if to == from {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Synapse \"{}\" is already at that position.", payload.id));
    }
    let item = scene.synapses.remove(from);
    scene.synapses.insert(to, item);
    protocol::MutationOutcome::new(diff_replace_content(scene.widgets, scene.synapses, scene.layout))
}
