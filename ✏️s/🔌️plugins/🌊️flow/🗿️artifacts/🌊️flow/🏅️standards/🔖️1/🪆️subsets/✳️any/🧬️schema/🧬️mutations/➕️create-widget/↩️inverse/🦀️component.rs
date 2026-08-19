//! ↩️ Undo mutation for `create-widget`: `delete-widget` by the created widget's own id.
use crate::artifacts::flow::schema::mutations::delete_widget::mutation::DeleteWidget;
use crate::artifacts::flow::schema::mutations::FlowMutation;
use crate::artifacts::flow::FlowSnapshot;
use protocol::Identified;

use super::mutation::CreateWidget;

pub async fn inverse(payload: &CreateWidget, _base: &FlowSnapshot) -> Vec<FlowMutation> {
    vec![FlowMutation::DeleteWidget(DeleteWidget { id: payload.widget.id().clone() })]
}
