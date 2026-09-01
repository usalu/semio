//! 🔺️ Sparse `FlowDiff` construction for `connect-widgets` — a real append-only synapse insert
//! against the current working scene (never a whole-snapshot capture).
use crate::artifacts::flow::schema::diff::text::{diff_replace_content, FlowDiff};
use crate::artifacts::flow::{flow_working_scene, FlowSnapshot};
use flow::SynapseSpec;
use protocol::Identified;

use super::ConnectWidgets;

pub fn diff(payload: &ConnectWidgets, base: &FlowSnapshot) -> protocol::MutationOutcome<FlowDiff> {
    let mut scene = flow_working_scene(base);
    if scene.synapses.iter().any(|synapse| synapse.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A synapse with id \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if !scene.widgets.iter().any(|widget| widget.id() == &payload.from) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Source widget \"{}\" does not exist.", payload.from), [payload.from.clone()]);
    }
    if !scene.widgets.iter().any(|widget| widget.id() == &payload.to) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Target widget \"{}\" does not exist.", payload.to), [payload.to.clone()]);
    }
    if scene.synapses.iter().any(|synapse| synapse.from == payload.from && synapse.from_port == payload.from_port && synapse.to == payload.to && synapse.to_port == payload.to_port) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("\"{}\"@{} is already connected to \"{}\"@{}; parallel synapses are not allowed.", payload.from, payload.from_port, payload.to, payload.to_port));
    }
    let synapse = SynapseSpec { id: payload.id.clone(), from: payload.from.clone(), from_port: payload.from_port.clone(), to: payload.to.clone(), to_port: payload.to_port.clone() };
    let index = payload.index.min(scene.synapses.len());
    scene.synapses.insert(index, synapse);
    protocol::MutationOutcome::new(diff_replace_content(scene.widgets, scene.synapses, scene.layout))
}
