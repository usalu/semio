//! 🔀️ Repositions a widget within the ordered widget list (never spatial — see `move-widgets`).
use crate::artifacts::flow::schema::mutations::FlowMutation;
use crate::artifacts::flow::{FlowDiff, FlowSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔀️ReorderWidgets
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReorderWidgets {
    pub id: String,
    pub to_index: usize,
}

impl MutationKind<FlowSnapshot, FlowMutation> for ReorderWidgets {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "reorder", entity: "widget", kind: "reorder-widgets", record: "ReorderedWidgets" };

    async fn diff(&self, base: &FlowSnapshot) -> protocol::MutationOutcome<FlowDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &FlowSnapshot) -> Vec<FlowMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Reorder widget \"{}\" to {}", self.id, self.to_index)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔀️ReorderWidgets
