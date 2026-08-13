//! 🔺️ Sparse `FlowDiff` construction for `replace-widget` — a real whole-value patch against the
//! current working scene (never a whole-snapshot capture).
use crate::artifacts::flow::schema::diff::text::{diff_replace_content, FlowDiff};
use crate::artifacts::flow::{flow_working_scene, FlowSnapshot};
use protocol::Identified;

use super::mutation::ReplaceWidget;

pub fn diff(payload: &ReplaceWidget, base: &FlowSnapshot) -> FlowDiff {
    let mut scene = flow_working_scene(base);
    if let Some(widget) = scene.widgets.iter_mut().find(|widget| widget.id() == &payload.id) {
        *widget = payload.widget.clone();
    }
    diff_replace_content(scene.widgets, scene.synapses, scene.layout)
}
