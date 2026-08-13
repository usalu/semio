//! 🔺️ Sparse `FlowDiff` construction for `connect-widgets` — a real append-only synapse insert
//! against the current working scene (never a whole-snapshot capture).
use crate::artifacts::flow::schema::diff::text::{diff_replace_content, FlowDiff};
use crate::artifacts::flow::{flow_working_scene, FlowSnapshot};
use flow::SynapseSpec;

use super::mutation::ConnectWidgets;

pub fn diff(payload: &ConnectWidgets, base: &FlowSnapshot) -> FlowDiff {
    let mut scene = flow_working_scene(base);
    let synapse = SynapseSpec { id: payload.id.clone(), from: payload.from.clone(), from_port: payload.from_port.clone(), to: payload.to.clone(), to_port: payload.to_port.clone() };
    let index = payload.index.min(scene.synapses.len());
    scene.synapses.insert(index, synapse);
    diff_replace_content(scene.widgets, scene.synapses, scene.layout)
}
