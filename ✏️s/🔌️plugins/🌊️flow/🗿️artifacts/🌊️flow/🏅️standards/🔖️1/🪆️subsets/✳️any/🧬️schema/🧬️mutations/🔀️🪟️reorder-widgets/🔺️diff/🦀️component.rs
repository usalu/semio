//! 🔺️ Sparse `FlowDiff` construction for `reorder-widgets` — recomputes the widget order from the
//! current working scene directly (never a whole-snapshot capture).
use crate::artifacts::flow::schema::diff::text::{diff_replace_content, FlowDiff};
use crate::artifacts::flow::{flow_working_scene, FlowSnapshot};
use protocol::Identified;

use super::mutation::ReorderWidgets;

pub fn diff(payload: &ReorderWidgets, base: &FlowSnapshot) -> FlowDiff {
    let mut scene = flow_working_scene(base);
    if let Some(from) = scene.widgets.iter().position(|widget| widget.id() == &payload.id) {
        let item = scene.widgets.remove(from);
        let to = payload.to_index.min(scene.widgets.len());
        scene.widgets.insert(to, item);
    }
    diff_replace_content(scene.widgets, scene.synapses, scene.layout)
}
