//! 🔺️ Sparse `FlowDiff` construction for `delete-widget`. Cascades into severed synapses and the
//! widget's layout entry (taxonomy `delete` — "captures cascade") against the current working scene
//! (never a whole-snapshot capture).
use crate::artifacts::flow::schema::diff::text::{diff_replace_content, FlowDiff};
use crate::artifacts::flow::{flow_working_scene, FlowSnapshot};
use protocol::Identified;

use super::mutation::DeleteWidget;

pub fn diff(payload: &DeleteWidget, base: &FlowSnapshot) -> FlowDiff {
    let mut scene = flow_working_scene(base);
    scene.widgets.retain(|widget| widget.id() != &payload.id);
    scene.synapses.retain(|synapse| synapse.from != payload.id && synapse.to != payload.id);
    scene.layout.remove(&payload.id);
    diff_replace_content(scene.widgets, scene.synapses, scene.layout)
}
