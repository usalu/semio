//! 🔺️ Sparse `FlowDiff` construction for `reorder-synapses` — recomputes the synapse order from the
//! current working scene directly (never a whole-snapshot capture).
use crate::artifacts::flow::schema::diff::text::{diff_replace_content, FlowDiff};
use crate::artifacts::flow::{flow_working_scene, FlowSnapshot};

use super::mutation::ReorderSynapses;

pub fn diff(payload: &ReorderSynapses, base: &FlowSnapshot) -> FlowDiff {
    let mut scene = flow_working_scene(base);
    if let Some(from) = scene.synapses.iter().position(|synapse| synapse.id == payload.id) {
        let item = scene.synapses.remove(from);
        let to = payload.to_index.min(scene.synapses.len());
        scene.synapses.insert(to, item);
    }
    diff_replace_content(scene.widgets, scene.synapses, scene.layout)
}
