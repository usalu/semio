//! 🗑️ Wires mutation — `DeleteNode`: removes one board node by id (cascade edges are separate
//! `disconnect-nodes` mutations, per the app's own delete-selection command).

use crate::artifacts::wires::diff::{board_after_remove_node, diff_board_fixture, WiresDiff};
use crate::artifacts::wires::mutations::WiresMutation;
use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::artifacts::wires::WiresSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🗑️ `delete-node` payload — the node's id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-node")]
pub struct DeleteNode {
    pub node_id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn delete_node(node_id: String) -> WiresMutation {
    WiresMutation::DeleteNode(DeleteNode { node_id })
}

impl protocol::MutationKind<WiresSnapshot, WiresMutation> for DeleteNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "node", kind: "delete-node", record: "DeletedNode" };

    async fn diff(&self, base: &WiresSnapshot) -> protocol::MutationOutcome<WiresDiff> {
        super::diff::diff(self, base).await
    }
    async fn inverse(&self, base: &WiresSnapshot) -> Vec<WiresMutation> {
        super::inverse::inverse(self, base).await
    }
    async fn label(&self) -> String {
        format!("Delete node \"{}\"", self.node_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.node_id.clone()]
    }
}
//#endregion 🔖️Mutation
