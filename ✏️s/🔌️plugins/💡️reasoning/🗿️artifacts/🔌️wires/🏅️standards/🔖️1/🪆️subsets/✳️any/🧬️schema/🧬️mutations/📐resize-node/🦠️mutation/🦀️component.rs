//! 📐️ Wires mutation — `ResizeNode`: changes one board node's extent — `radius` for a `circle`
//! node, `width`/`height` for a `rectangle` node (`NodeDsl`'s own optional-field shape, mirrored
//! here as the mutation's own extent facet per `📓️taxonomy.md`'s `resize` verb).
use crate::artifacts::wires::diff::WiresDiff;
use crate::artifacts::wires::mutations::WiresMutation;
use crate::artifacts::wires::WiresSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📐️ `resize-node` payload — only the extent fields actually being changed are `Some`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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
pub fn resize_node(node_id: String, new_radius: Option<f64>, new_width: Option<f64>, new_height: Option<f64>) -> WiresMutation {
    WiresMutation::ResizeNode(ResizeNode { node_id, new_radius, new_width, new_height })
}

impl protocol::MutationKind<WiresSnapshot, WiresMutation> for ResizeNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "resize", entity: "node", kind: "resize-node", record: "ResizedNode" };

    fn diff(&self, base: &WiresSnapshot) -> WiresDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &WiresSnapshot) -> Vec<WiresMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Resize node \"{}\"", self.node_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.node_id.clone()]
    }
}
//#endregion 🔖️Mutation
