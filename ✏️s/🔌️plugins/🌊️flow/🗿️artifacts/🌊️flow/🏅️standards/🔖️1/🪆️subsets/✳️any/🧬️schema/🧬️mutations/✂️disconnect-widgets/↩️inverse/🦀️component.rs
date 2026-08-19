//! ↩️ Undo mutation for `disconnect-widgets`: re-`connect-widgets` at the base-state index, carrying
//! the full removed payload (taxonomy `## Addressing convention` §5).
use crate::artifacts::flow::schema::mutations::connect_widgets::mutation::ConnectWidgets;
use crate::artifacts::flow::schema::mutations::FlowMutation;
use crate::artifacts::flow::{flow_working_scene, FlowSnapshot};

use super::mutation::DisconnectWidgets;

pub async fn inverse(payload: &DisconnectWidgets, base: &FlowSnapshot) -> Vec<FlowMutation> {
    let scene = flow_working_scene(base);
    let Some(index) = scene.synapses.iter().position(|synapse| synapse.id == payload.id) else {
        return Vec::new();
    };
    let synapse = &scene.synapses[index];
    vec![FlowMutation::ConnectWidgets(ConnectWidgets {
        index,
        id: synapse.id.clone(),
        from: synapse.from.clone(),
        from_port: synapse.from_port.clone(),
        to: synapse.to.clone(),
        to_port: synapse.to_port.clone(),
    })]
}
