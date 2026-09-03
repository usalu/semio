//! ✂️ Wires mutation — `DisconnectNodes`: removes a board edge (and its wires-level relationship,
//! if any) by the edge's id.

use crate::artifacts::wires::diff::{diff_wires_and_board, fixtures_after_remove_edge, WiresDiff};
use crate::artifacts::wires::mutations::WiresMutation;
use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::{find_board_edge, find_relationship};
use crate::artifacts::wires::WiresSnapshot;
use dsl::DslValue;

//#region 🔖️Mutation
/// ✂️ `disconnect-nodes` payload — the edge's id.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "disconnect-nodes")]
pub struct DisconnectNodes {
    pub edge_id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn disconnect_nodes(edge_id: String) -> WiresMutation {
    WiresMutation::DisconnectNodes(DisconnectNodes { edge_id })
}

impl protocol::MutationKind<WiresSnapshot, WiresMutation> for DisconnectNodes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "disconnect", entity: "relationship", kind: "disconnect-nodes", record: "DisconnectedNodes" };

    async fn diff(&self, base: &WiresSnapshot) -> protocol::MutationOutcome<WiresDiff> {
        super::diff::diff(self, base).await
    }
    async fn inverse(&self, base: &WiresSnapshot) -> Vec<WiresMutation> {
        super::inverse::inverse(self, base).await
    }
    async fn label(&self) -> String {
        format!("Disconnect edge \"{}\"", self.edge_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.edge_id.clone()]
    }
}
//#endregion 🔖️Mutation
