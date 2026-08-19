//! 🔗 Wires mutation — `ConnectNodes`: creates a board edge between two nodes, plus its optional
//! wires-level semantic relationship (`relationship` is `DslValue::Null` when the edge carries no
//! wires relationship — the pre-migration fixture's own convention, preserved as-is).
use crate::artifacts::wires::diff::WiresDiff;
use crate::artifacts::wires::schema::entity_id;
use crate::artifacts::wires::mutations::WiresMutation;
use crate::artifacts::wires::WiresSnapshot;
use dsl::DslValue;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔗 `connect-nodes` payload — the full new board edge, plus its (possibly null) relationship.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "connect-nodes")]
pub struct ConnectNodes {
    pub edge: DslValue,
    pub relationship: DslValue,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn connect_nodes(edge: DslValue, relationship: DslValue) -> WiresMutation {
    WiresMutation::ConnectNodes(ConnectNodes { edge, relationship })
}

impl protocol::MutationKind<WiresSnapshot, WiresMutation> for ConnectNodes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "connect", entity: "relationship", kind: "connect-nodes", record: "ConnectedNodes" };

    async fn diff(&self, base: &WiresSnapshot) -> protocol::MutationOutcome<WiresDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &WiresSnapshot) -> Vec<WiresMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Connect nodes via edge \"{}\"", entity_id(&self.edge, "id").unwrap_or("?"))
    }
    async fn target(&self) -> Vec<String> {
        entity_id(&self.edge, "id").map(|id| vec![id.to_string()]).unwrap_or_default()
    }
}
//#endregion 🔖️Mutation
