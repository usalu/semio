//! 🔺️ Sparse `FlowDiff` construction for `update-synapse-endpoints` — a real whole-endpoints patch
//! against the current working scene (never a whole-snapshot capture).
use crate::artifacts::flow::schema::diff::text::{diff_replace_content, FlowDiff};
use crate::artifacts::flow::{flow_working_scene, FlowSnapshot};
use protocol::Identified;

use super::mutation::UpdateSynapseEndpoints;

pub async fn diff(payload: &UpdateSynapseEndpoints, base: &FlowSnapshot) -> protocol::MutationOutcome<FlowDiff> {
    let mut scene = flow_working_scene(base);
    if !scene.synapses.iter().any(|synapse| synapse.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Synapse \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if !scene.widgets.iter().any(|widget| widget.id() == &payload.from) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Source widget \"{}\" does not exist.", payload.from), [payload.from.clone()]);
    }
    if !scene.widgets.iter().any(|widget| widget.id() == &payload.to) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Target widget \"{}\" does not exist.", payload.to), [payload.to.clone()]);
    }
    let synapse = scene.synapses.iter_mut().find(|synapse| synapse.id == payload.id).expect("presence confirmed above");
    if synapse.from == payload.from && synapse.from_port == payload.from_port && synapse.to == payload.to && synapse.to_port == payload.to_port {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Synapse \"{}\" already has those endpoints.", payload.id));
    }
    synapse.from = payload.from.clone();
    synapse.from_port = payload.from_port.clone();
    synapse.to = payload.to.clone();
    synapse.to_port = payload.to_port.clone();
    protocol::MutationOutcome::new(diff_replace_content(scene.widgets, scene.synapses, scene.layout))
}
