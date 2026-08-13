//! 🔺️ Sparse `FlowDiff` construction for `update-synapse-endpoints` — a real whole-endpoints patch
//! against the current working scene (never a whole-snapshot capture).
use crate::artifacts::flow::schema::diff::text::{diff_replace_content, FlowDiff};
use crate::artifacts::flow::{flow_working_scene, FlowSnapshot};

use super::mutation::UpdateSynapseEndpoints;

pub fn diff(payload: &UpdateSynapseEndpoints, base: &FlowSnapshot) -> FlowDiff {
    let mut scene = flow_working_scene(base);
    if let Some(synapse) = scene.synapses.iter_mut().find(|synapse| synapse.id == payload.id) {
        synapse.from = payload.from.clone();
        synapse.from_port = payload.from_port.clone();
        synapse.to = payload.to.clone();
        synapse.to_port = payload.to_port.clone();
    }
    diff_replace_content(scene.widgets, scene.synapses, scene.layout)
}
