//! ↩️ Undo mutation for `delete-widget`: re-`create`s the widget at its base-state index, restores
//! its layout entry, then re-`connect`s severed synapses in reverse dependency order (taxonomy
//! `## Addressing convention` §5 — inverse always computed from `base`, never by inverting the diff
//! structurally).
use crate::artifacts::flow::schema::mutations::connect_widgets::mutation::ConnectWidgets;
use crate::artifacts::flow::schema::mutations::create_widget::mutation::CreateWidget;
use crate::artifacts::flow::schema::mutations::move_widgets::mutation::MoveWidgets;
use crate::artifacts::flow::schema::mutations::FlowMutation;
use crate::artifacts::flow::{flow_working_scene, FlowSnapshot};
use flow::FlowLayoutEntry;
use protocol::Identified;

use super::mutation::DeleteWidget;

pub async fn inverse(payload: &DeleteWidget, base: &FlowSnapshot) -> Vec<FlowMutation> {
    let scene = flow_working_scene(base);
    let Some(index) = scene.widgets.iter().position(|widget| widget.id() == &payload.id) else {
        return Vec::new();
    };
    let widget = scene.widgets[index].clone();
    let mut inverses = vec![FlowMutation::CreateWidget(CreateWidget { index, widget })];

    if let Some(layout) = scene.layout.get(&payload.id) {
        inverses.push(FlowMutation::MoveWidgets(MoveWidgets {
            entries: vec![FlowLayoutEntry { id: payload.id.clone(), layout: Some(layout.clone()) }],
        }));
    }

    let severed_indices: Vec<usize> = scene
        .synapses
        .iter()
        .enumerate()
        .filter(|(_, synapse)| synapse.from == payload.id || synapse.to == payload.id)
        .map(|(index, _)| index)
        .collect();
    for &synapse_index in severed_indices.iter().rev() {
        let synapse = &scene.synapses[synapse_index];
        inverses.push(FlowMutation::ConnectWidgets(ConnectWidgets {
            index: synapse_index,
            id: synapse.id.clone(),
            from: synapse.from.clone(),
            from_port: synapse.from_port.clone(),
            to: synapse.to.clone(),
            to_port: synapse.to_port.clone(),
        }));
    }

    inverses
}
