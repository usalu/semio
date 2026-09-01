//! ↩️ Undo mutation for `replace-widget`: replace back with the widget's prior `base` value.
use crate::artifacts::flow::schema::mutations::FlowMutation;
use crate::artifacts::flow::{flow_working_scene, FlowSnapshot};
use protocol::Identified;

use super::ReplaceWidget;

pub fn inverse(payload: &ReplaceWidget, base: &FlowSnapshot) -> Vec<FlowMutation> {
    let scene = flow_working_scene(base);
    match scene.widgets.iter().find(|widget| widget.id() == &payload.id) {
        Some(previous) => vec![FlowMutation::ReplaceWidget(ReplaceWidget { id: payload.id.clone(), widget: previous.clone() })],
        None => Vec::new(),
    }
}
