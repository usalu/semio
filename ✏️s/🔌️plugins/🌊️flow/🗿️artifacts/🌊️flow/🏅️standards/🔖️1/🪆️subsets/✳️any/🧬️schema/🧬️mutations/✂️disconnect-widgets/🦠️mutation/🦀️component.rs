//! ✂️ Removes a synapse edge by id.
use crate::artifacts::flow::schema::mutations::FlowMutation;
use crate::artifacts::flow::{FlowDiff, FlowSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region ✂️DisconnectWidgets
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisconnectWidgets {
    pub id: String,
}

impl MutationKind<FlowSnapshot, FlowMutation> for DisconnectWidgets {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "disconnect", entity: "synapse", kind: "disconnect-widgets", record: "DisconnectedWidgets" };

    async fn diff(&self, base: &FlowSnapshot) -> protocol::MutationOutcome<FlowDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &FlowSnapshot) -> Vec<FlowMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Disconnect \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion ✂️DisconnectWidgets
