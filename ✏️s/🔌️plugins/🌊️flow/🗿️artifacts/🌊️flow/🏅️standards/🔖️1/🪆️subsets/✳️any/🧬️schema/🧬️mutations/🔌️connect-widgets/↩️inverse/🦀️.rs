//! ↩️ Undo mutation for `connect-widgets`: `disconnect-widgets` by the created synapse's own id.
use crate::schema::mutations::disconnect_widgets::DisconnectWidgets;
use crate::schema::mutations::FlowMutation;
use crate::FlowSnapshot;

use super::ConnectWidgets;

pub fn inverse(payload: &ConnectWidgets, _base: &FlowSnapshot) -> Vec<FlowMutation> {
    vec![FlowMutation::DisconnectWidgets(DisconnectWidgets { id: payload.id.clone() })]
}
