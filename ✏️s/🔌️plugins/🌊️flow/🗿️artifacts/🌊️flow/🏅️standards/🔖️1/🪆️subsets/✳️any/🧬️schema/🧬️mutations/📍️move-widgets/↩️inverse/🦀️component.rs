//! ↩️ Undo mutation for `move-widgets`, restoring each entry's prior `base.layout` value.
use crate::artifacts::flow::schema::mutations::FlowMutation;
use crate::artifacts::flow::FlowSnapshot;
use flow::FlowLayoutEntry;

use super::mutation::MoveWidgets;

pub fn inverse(payload: &MoveWidgets, base: &FlowSnapshot) -> Vec<FlowMutation> {
    if payload.entries.is_empty() {
        return Vec::new();
    }
    let entries = payload
        .entries
        .iter()
        .map(|entry| FlowLayoutEntry { id: entry.id.clone(), layout: base.layout.get(&entry.id).cloned() })
        .collect();
    vec![FlowMutation::MoveWidgets(MoveWidgets { entries })]
}
