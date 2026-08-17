//! 🔺️ Sparse `FlowDiff` construction for `move-widgets` against the current working scene (never a
//! whole-snapshot capture).
use crate::artifacts::flow::schema::diff::text::{diff_replace_content, FlowDiff};
use crate::artifacts::flow::{flow_working_scene, FlowSnapshot};
use protocol::Identified;

use super::mutation::MoveWidgets;

pub fn diff(payload: &MoveWidgets, base: &FlowSnapshot) -> protocol::MutationOutcome<FlowDiff> {
    let mut scene = flow_working_scene(base);
    let missing_ids: Vec<String> = payload.entries.iter().map(|entry| entry.id.clone()).filter(|id| !scene.widgets.iter().any(|widget| widget.id() == id)).collect();
    if !missing_ids.is_empty() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Widget(s) do not exist: {}.", missing_ids.join(", ")), missing_ids);
    }
    let non_finite_ids: Vec<String> = payload.entries.iter().filter(|entry| entry.layout.as_ref().is_some_and(|layout| !layout.x.is_finite() || !layout.y.is_finite())).map(|entry| entry.id.clone()).collect();
    if !non_finite_ids.is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Widget(s) have a non-finite layout position: {}.", non_finite_ids.join(", ")), non_finite_ids);
    }
    if payload.entries.iter().all(|entry| scene.layout.get(&entry.id) == entry.layout.as_ref()) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Widget position(s) already match the requested layout.");
    }
    for entry in &payload.entries {
        match &entry.layout {
            Some(layout) => {
                scene.layout.insert(entry.id.clone(), layout.clone());
            }
            None => {
                scene.layout.remove(&entry.id);
            }
        }
    }
    protocol::MutationOutcome::new(diff_replace_content(scene.widgets, scene.synapses, scene.layout))
}
