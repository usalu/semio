//! 🔺️ Sparse `FlowDiff` construction for `disconnect-widgets` — a real synapse removal against the
//! current working scene (never a whole-snapshot capture).
use crate::artifacts::flow::schema::diff::text::{diff_replace_content, FlowDiff};
use crate::artifacts::flow::{flow_working_scene, FlowSnapshot};

use super::DisconnectWidgets;

pub fn diff(payload: &DisconnectWidgets, base: &FlowSnapshot) -> protocol::MutationOutcome<FlowDiff> {
    let mut scene = flow_working_scene(base);
    if !scene.synapses.iter().any(|synapse| synapse.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Synapse \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    scene.synapses.retain(|synapse| synapse.id != payload.id);
    protocol::MutationOutcome::new(diff_replace_content(scene.widgets, scene.synapses, scene.layout))
}
