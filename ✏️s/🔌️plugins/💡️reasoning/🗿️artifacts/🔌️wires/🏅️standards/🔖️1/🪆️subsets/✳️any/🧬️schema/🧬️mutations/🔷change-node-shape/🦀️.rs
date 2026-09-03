//! 🔷 Wires mutation — `ChangeNodeShape`: sets one board node's `shape` scalar field
//! (`"circle"`/`"rectangle"`, per `NodeDsl`'s doc).

use crate::artifacts::wires::diff::{diff_board_fixture, WiresDiff};
use crate::artifacts::wires::mutations::{set_node_field, WiresMutation};
use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::artifacts::wires::WiresSnapshot;
use dsl::DslValue;

//#region 🔖️Mutation
/// 🔷 `change-node-shape` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-node-shape")]
pub struct ChangeNodeShape {
    pub node_id: String,
    pub new_shape: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_node_shape(node_id: String, new_shape: String) -> WiresMutation {
    WiresMutation::ChangeNodeShape(ChangeNodeShape { node_id, new_shape })
}

impl protocol::MutationKind<WiresSnapshot, WiresMutation> for ChangeNodeShape {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node", kind: "change-node-shape", record: "ChangedNodeShape" };

    async fn diff(&self, base: &WiresSnapshot) -> protocol::MutationOutcome<WiresDiff> {
        super::diff::diff(self, base).await
    }
    async fn inverse(&self, base: &WiresSnapshot) -> Vec<WiresMutation> {
        super::inverse::inverse(self, base).await
    }
    async fn label(&self) -> String {
        format!("Change node \"{}\" shape to \"{}\"", self.node_id, self.new_shape)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.node_id.clone()]
    }
}
//#endregion 🔖️Mutation
