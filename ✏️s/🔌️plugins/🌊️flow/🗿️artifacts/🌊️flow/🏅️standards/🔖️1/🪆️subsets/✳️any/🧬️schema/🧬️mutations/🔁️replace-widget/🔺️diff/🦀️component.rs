//! 🔺️ Sparse `FlowDiff` construction for `replace-widget` — a real whole-value patch against the
//! current working scene (never a whole-snapshot capture).
use crate::artifacts::flow::schema::diff::text::{diff_replace_content, FlowDiff};
use crate::artifacts::flow::{flow_working_scene, FlowSnapshot};
use protocol::Identified;

use super::mutation::ReplaceWidget;

pub async fn diff(payload: &ReplaceWidget, base: &FlowSnapshot) -> protocol::MutationOutcome<FlowDiff> {
    let mut scene = flow_working_scene(base);
    let Some(widget) = scene.widgets.iter_mut().find(|widget| widget.id() == &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Widget \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if *widget == payload.widget {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Widget \"{}\" already matches the requested value.", payload.id));
    }
    *widget = payload.widget.clone();
    protocol::MutationOutcome::new(diff_replace_content(scene.widgets, scene.synapses, scene.layout))
}
