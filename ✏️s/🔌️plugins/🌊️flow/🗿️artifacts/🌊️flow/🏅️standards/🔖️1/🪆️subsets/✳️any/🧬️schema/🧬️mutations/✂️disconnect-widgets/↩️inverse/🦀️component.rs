//! ↩️ Undo mutation for `disconnect-widgets`: re-`connect-widgets` at the base-state index, carrying
//! the full removed payload (taxonomy `## Addressing convention` §5).
use crate::artifacts::flow::schema::mutations::connect_widgets::mutation::ConnectWidgets;
use crate::artifacts::flow::schema::mutations::FlowMutation;
use crate::artifacts::flow::FlowSnapshot;

use super::mutation::DisconnectWidgets;

pub fn inverse(payload: &DisconnectWidgets, base: &FlowSnapshot) -> Vec<FlowMutation> {
    let Some(index) = base.synapses.iter().position(|synapse| synapse.id == payload.id) else {
        return Vec::new();
    };
    let synapse = &base.synapses[index];
    vec![FlowMutation::ConnectWidgets(ConnectWidgets {
        index,
        id: synapse.id.clone(),
        from: synapse.from.clone(),
        from_port: synapse.from_port.clone(),
        to: synapse.to.clone(),
        to_port: synapse.to_port.clone(),
    })]
}
