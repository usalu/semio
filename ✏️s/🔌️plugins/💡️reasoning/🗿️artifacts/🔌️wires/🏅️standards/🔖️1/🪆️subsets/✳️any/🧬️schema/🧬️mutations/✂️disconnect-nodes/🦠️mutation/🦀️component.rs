//! ✂️ Wires mutation — `DisconnectNodes`: removes a board edge (and its wires-level relationship,
//! if any) by the edge's id.
use crate::artifacts::wires::diff::WiresDiff;
use crate::artifacts::wires::mutations::WiresMutation;
use crate::artifacts::wires::WiresSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ✂️ `disconnect-nodes` payload — the edge's id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
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
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &WiresSnapshot) -> Vec<WiresMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Disconnect edge \"{}\"", self.edge_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.edge_id.clone()]
    }
}
//#endregion 🔖️Mutation
