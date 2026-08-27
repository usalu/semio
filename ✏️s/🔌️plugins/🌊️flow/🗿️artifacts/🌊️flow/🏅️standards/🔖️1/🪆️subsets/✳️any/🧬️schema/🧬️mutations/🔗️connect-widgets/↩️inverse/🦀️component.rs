//! ↩️ Undo mutation for `connect-widgets`: `disconnect-widgets` by the created synapse's own id.
use crate::artifacts::flow::schema::mutations::disconnect_widgets::mutation::DisconnectWidgets;
use crate::artifacts::flow::schema::mutations::FlowMutation;
use crate::artifacts::flow::FlowSnapshot;

use super::mutation::ConnectWidgets;

pub fn inverse(payload: &ConnectWidgets, _base: &FlowSnapshot) -> Vec<FlowMutation> {
    vec![FlowMutation::DisconnectWidgets(DisconnectWidgets { id: payload.id.clone() })]
}
