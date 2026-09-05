//! 🔺️ Sparse `FlowDiff` construction for `reorder-widgets` — recomputes the widget order from the
//! current working scene directly (never a whole-snapshot capture).
use crate::artifacts::flow::schema::diff::text::{diff_replace_content, FlowDiff};
use crate::artifacts::flow::{flow_working_scene, FlowSnapshot};
use protocol::Identified;

use super::ReorderWidgets;

pub fn diff(payload: &ReorderWidgets, base: &FlowSnapshot) -> protocol::MutationOutcome<FlowDiff> {
    let mut scene = flow_working_scene(base);
    let Some(from) = scene.widgets.iter().position(|widget| widget.id() == &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Widget \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let to = payload.to_index.min(scene.widgets.len().saturating_sub(1));
    if to == from {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Widget \"{}\" is already at that position.", payload.id));
    }
    let item = scene.widgets.remove(from);
    scene.widgets.insert(to, item);
    protocol::MutationOutcome::new(diff_replace_content(scene.widgets, scene.synapses, scene.layout))
}
