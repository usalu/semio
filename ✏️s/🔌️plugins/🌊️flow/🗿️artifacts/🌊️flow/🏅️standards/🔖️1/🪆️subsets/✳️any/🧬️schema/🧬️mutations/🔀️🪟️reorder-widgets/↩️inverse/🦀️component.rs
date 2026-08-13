//! ↩️ Undo mutation for `reorder-widgets`: reorder back to the base-state index.
use crate::artifacts::flow::schema::mutations::FlowMutation;
use crate::artifacts::flow::{flow_working_scene, FlowSnapshot};
use protocol::Identified;

use super::mutation::ReorderWidgets;

pub fn inverse(payload: &ReorderWidgets, base: &FlowSnapshot) -> Vec<FlowMutation> {
    let scene = flow_working_scene(base);
    let Some(original_index) = scene.widgets.iter().position(|widget| widget.id() == &payload.id) else {
        return Vec::new();
    };
    vec![FlowMutation::ReorderWidgets(ReorderWidgets { id: payload.id.clone(), to_index: original_index })]
}
