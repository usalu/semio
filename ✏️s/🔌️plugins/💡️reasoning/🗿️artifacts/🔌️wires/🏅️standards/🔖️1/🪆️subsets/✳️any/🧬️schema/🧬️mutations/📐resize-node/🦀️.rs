//! 📐️ Wires mutation — `ResizeNode`: changes one board node's extent — `radius` for a `circle`
//! node, `width`/`height` for a `rectangle` node (`NodeDsl`'s own optional-field shape, mirrored
//! here as the mutation's own extent facet per `📓️taxonomy.md`'s `resize` verb).

use crate::artifacts::wires::diff::{diff_board_fixture, WiresDiff};
use crate::artifacts::wires::mutations::{set_node_field, WiresMutation};
use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::artifacts::wires::WiresSnapshot;
use dsl::DslValue;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📐️ `resize-node` payload — only the extent fields actually being changed are `Some`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "resize-node")]
pub struct ResizeNode {
    pub node_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_radius: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_width: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_height: Option<f64>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn resize_node(node_id: String, new_radius: Option<f64>, new_width: Option<f64>, new_height: Option<f64>) -> WiresMutation {
    WiresMutation::ResizeNode(ResizeNode { node_id, new_radius, new_width, new_height })
}

impl protocol::MutationKind<WiresSnapshot, WiresMutation> for ResizeNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "resize", entity: "node", kind: "resize-node", record: "ResizedNode" };

    async fn diff(&self, base: &WiresSnapshot) -> protocol::MutationOutcome<WiresDiff> {
        super::diff::diff(self, base).await
    }
    async fn inverse(&self, base: &WiresSnapshot) -> Vec<WiresMutation> {
        super::inverse::inverse(self, base).await
    }
    async fn label(&self) -> String {
        format!("Resize node \"{}\"", self.node_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.node_id.clone()]
    }
}
//#endregion 🔖️Mutation
