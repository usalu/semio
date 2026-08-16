//! 🏷️ Wires mutation — `ChangeNodeKind`: sets one board node's `nodeKind` scalar field.
use crate::artifacts::wires::diff::WiresDiff;
use crate::artifacts::wires::mutations::WiresMutation;
use crate::artifacts::wires::WiresSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🏷️ `change-node-kind` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-node-kind")]
pub struct ChangeNodeKind {
    pub node_id: String,
    pub new_node_kind: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_node_kind(node_id: String, new_node_kind: String) -> WiresMutation {
    WiresMutation::ChangeNodeKind(ChangeNodeKind { node_id, new_node_kind })
}

impl protocol::MutationKind<WiresSnapshot, WiresMutation> for ChangeNodeKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node", kind: "change-node-kind", record: "ChangedNodeKind" };

    fn diff(&self, base: &WiresSnapshot) -> protocol::MutationOutcome<WiresDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &WiresSnapshot) -> Vec<WiresMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change node \"{}\" kind to \"{}\"", self.node_id, self.new_node_kind)
    }
    fn target(&self) -> Vec<String> {
        vec![self.node_id.clone()]
    }
}
//#endregion 🔖️Mutation
