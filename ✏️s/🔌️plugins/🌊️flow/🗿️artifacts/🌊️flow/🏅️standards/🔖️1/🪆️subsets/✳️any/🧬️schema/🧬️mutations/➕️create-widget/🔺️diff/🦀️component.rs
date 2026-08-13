//! 🔺️ Sparse `FlowDiff` construction for `create-widget` — a real append-only insert against the
//! current working scene (never a whole-snapshot capture).
use crate::artifacts::flow::schema::diff::text::{diff_replace_content, FlowDiff};
use crate::artifacts::flow::{flow_working_scene, FlowSnapshot};

use super::mutation::CreateWidget;

pub fn diff(payload: &CreateWidget, base: &FlowSnapshot) -> FlowDiff {
    let mut scene = flow_working_scene(base);
    let index = payload.index.min(scene.widgets.len());
    scene.widgets.insert(index, payload.widget.clone());
    diff_replace_content(scene.widgets, scene.synapses, scene.layout)
}
