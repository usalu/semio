//! 🔺️ Sparse `FlowDiff` construction for `move-widgets` against the current working scene (never a
//! whole-snapshot capture).
use crate::artifacts::flow::schema::diff::text::{diff_replace_content, FlowDiff};
use crate::artifacts::flow::{flow_working_scene, FlowSnapshot};

use super::mutation::MoveWidgets;

pub fn diff(payload: &MoveWidgets, base: &FlowSnapshot) -> FlowDiff {
    let mut scene = flow_working_scene(base);
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
    diff_replace_content(scene.widgets, scene.synapses, scene.layout)
}
