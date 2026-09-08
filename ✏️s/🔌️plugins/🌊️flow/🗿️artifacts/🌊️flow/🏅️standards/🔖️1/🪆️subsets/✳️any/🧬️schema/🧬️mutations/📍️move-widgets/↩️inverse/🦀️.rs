//! ↩️ Undo mutation for `move-widgets`, restoring each entry's prior `base.layout` value.
use crate::schema::mutations::FlowMutation;
use crate::{flow_working_scene, FlowSnapshot};
use semio_framework_artifact_flow_flow::FlowLayoutEntry;

use super::MoveWidgets;

pub fn inverse(payload: &MoveWidgets, base: &FlowSnapshot) -> Vec<FlowMutation> {
    if payload.entries.is_empty() {
        return Vec::new();
    }
    let scene = flow_working_scene(base);
    let entries = payload.entries.iter().map(|entry| FlowLayoutEntry { id: entry.id.clone(), layout: scene.layout.get(&entry.id).cloned() }).collect();
    vec![FlowMutation::MoveWidgets(MoveWidgets { entries })]
}
