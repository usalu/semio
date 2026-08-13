//! 🔺️ Sparse `FlowDiff` construction for `disconnect-widgets` — a real synapse removal against the
//! current working scene (never a whole-snapshot capture).
use crate::artifacts::flow::schema::diff::text::{diff_replace_content, FlowDiff};
use crate::artifacts::flow::{flow_working_scene, FlowSnapshot};

use super::mutation::DisconnectWidgets;

pub fn diff(payload: &DisconnectWidgets, base: &FlowSnapshot) -> FlowDiff {
    let mut scene = flow_working_scene(base);
    scene.synapses.retain(|synapse| synapse.id != payload.id);
    diff_replace_content(scene.widgets, scene.synapses, scene.layout)
}
