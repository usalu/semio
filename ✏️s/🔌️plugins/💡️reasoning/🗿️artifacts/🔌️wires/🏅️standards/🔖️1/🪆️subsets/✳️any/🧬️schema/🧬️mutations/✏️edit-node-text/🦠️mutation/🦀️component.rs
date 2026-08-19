//! ✏️ Wires mutation — `EditNodeText`: replaces one board node's authored `text` label
//! (`📓️taxonomy.md`'s `edit` verb — an authored content body, not a bare scalar rename).
use crate::artifacts::wires::diff::WiresDiff;
use crate::artifacts::wires::mutations::WiresMutation;
use crate::artifacts::wires::WiresSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ✏️ `edit-node-text` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "edit-node-text")]
pub struct EditNodeText {
    pub node_id: String,
    pub new_text: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn edit_node_text(node_id: String, new_text: String) -> WiresMutation {
    WiresMutation::EditNodeText(EditNodeText { node_id, new_text })
}

impl protocol::MutationKind<WiresSnapshot, WiresMutation> for EditNodeText {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "node", kind: "edit-node-text", record: "EditedNodeText" };

    async fn diff(&self, base: &WiresSnapshot) -> protocol::MutationOutcome<WiresDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &WiresSnapshot) -> Vec<WiresMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Edit node \"{}\" text to \"{}\"", self.node_id, self.new_text)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.node_id.clone()]
    }
}
//#endregion 🔖️Mutation
