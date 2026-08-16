//! 🌱 Wires mutation — `CreateNode`: brings one board node into existence (full initial payload,
//! id-keyed per `📓️derivation-rules.md` rule 2).
use crate::artifacts::wires::diff::WiresDiff;
use crate::artifacts::wires::schema::entity_id;
use crate::artifacts::wires::mutations::WiresMutation;
use crate::artifacts::wires::WiresSnapshot;
use dsl::DslValue;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌱 `create-node` payload — the node's full initial state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-node")]
pub struct CreateNode {
    pub node: DslValue,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_node(node: DslValue) -> WiresMutation {
    WiresMutation::CreateNode(CreateNode { node })
}

impl protocol::MutationKind<WiresSnapshot, WiresMutation> for CreateNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "node", kind: "create-node", record: "CreatedNode" };

    fn diff(&self, base: &WiresSnapshot) -> protocol::MutationOutcome<WiresDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &WiresSnapshot) -> Vec<WiresMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Add node \"{}\"", entity_id(&self.node, "id").unwrap_or("?"))
    }
    fn target(&self) -> Vec<String> {
        entity_id(&self.node, "id").map(|id| vec![id.to_string()]).unwrap_or_default()
    }
}
//#endregion 🔖️Mutation
