//! 🔺️ Sparse `FlowDiff` construction for `create-widget` — a real append-only insert against the
//! current working scene (never a whole-snapshot capture).
use crate::artifacts::flow::schema::diff::text::{diff_replace_content, FlowDiff};
use crate::artifacts::flow::{flow_working_scene, FlowSnapshot};
use protocol::Identified;

use super::mutation::CreateWidget;

pub async fn diff(payload: &CreateWidget, base: &FlowSnapshot) -> protocol::MutationOutcome<FlowDiff> {
    let mut scene = flow_working_scene(base);
    if scene.widgets.iter().any(|widget| widget.id() == payload.widget.id()) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A widget with id \"{}\" already exists.", payload.widget.id()), [payload.widget.id().clone()]);
    }
    let index = payload.index.min(scene.widgets.len());
    scene.widgets.insert(index, payload.widget.clone());
    protocol::MutationOutcome::new(diff_replace_content(scene.widgets, scene.synapses, scene.layout))
}
