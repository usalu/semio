//! ↩️ Undo mutation for `move-widgets`, restoring each entry's prior `base.layout` value.
use crate::artifacts::flow::schema::mutations::FlowMutation;
use crate::artifacts::flow::{flow_working_scene, FlowSnapshot};
use flow::FlowLayoutEntry;

use super::MoveWidgets;

pub fn inverse(payload: &MoveWidgets, base: &FlowSnapshot) -> Vec<FlowMutation> {
    if payload.entries.is_empty() {
        return Vec::new();
    }
    let scene = flow_working_scene(base);
    let entries = payload.entries.iter().map(|entry| FlowLayoutEntry { id: entry.id.clone(), layout: scene.layout.get(&entry.id).cloned() }).collect();
    vec![FlowMutation::MoveWidgets(MoveWidgets { entries })]
}
