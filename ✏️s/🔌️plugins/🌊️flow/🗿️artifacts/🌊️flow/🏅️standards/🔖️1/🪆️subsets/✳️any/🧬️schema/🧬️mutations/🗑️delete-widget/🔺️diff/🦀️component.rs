//! 🔺️ Sparse `FlowDiff` construction for `delete-widget`. Cascades into severed synapses and the
//! widget's layout entry (taxonomy `delete` — "captures cascade") against the current working scene
//! (never a whole-snapshot capture).
use crate::artifacts::flow::schema::diff::text::{diff_replace_content, FlowDiff};
use crate::artifacts::flow::{flow_working_scene, FlowSnapshot};
use protocol::Identified;

use super::mutation::DeleteWidget;

pub async fn diff(payload: &DeleteWidget, base: &FlowSnapshot) -> protocol::MutationOutcome<FlowDiff> {
    let mut scene = flow_working_scene(base);
    if !scene.widgets.iter().any(|widget| widget.id() == &payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Widget \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let cascaded_synapse_ids: Vec<String> = scene.synapses.iter().filter(|synapse| synapse.from == payload.id || synapse.to == payload.id).map(|synapse| synapse.id.clone()).collect();
    scene.widgets.retain(|widget| widget.id() != &payload.id);
    scene.synapses.retain(|synapse| synapse.from != payload.id && synapse.to != payload.id);
    scene.layout.remove(&payload.id);
    let outcome = protocol::MutationOutcome::new(diff_replace_content(scene.widgets, scene.synapses, scene.layout));
    if cascaded_synapse_ids.is_empty() {
        outcome
    } else {
        outcome.info("mutation.cascade", format!("Deleting widget \"{}\" also removed {} connected synapse(s): {}.", payload.id, cascaded_synapse_ids.len(), cascaded_synapse_ids.join(", ")))
    }
}
