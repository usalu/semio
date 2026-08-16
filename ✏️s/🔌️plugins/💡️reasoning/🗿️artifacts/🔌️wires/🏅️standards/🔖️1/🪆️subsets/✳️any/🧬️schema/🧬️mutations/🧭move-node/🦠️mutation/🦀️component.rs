//! 🧭️ Wires mutation — `MoveNode`: absolute spatial reposition of one board node
//! (`📓️taxonomy.md`'s `move` verb). Replaces the old generic `PatchNode{x,y}` call sites
//! (force-layout, canvas drag).
use crate::artifacts::wires::diff::WiresDiff;
use crate::artifacts::wires::mutations::WiresMutation;
use crate::artifacts::wires::WiresSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🧭️ `move-node` payload — the node's new absolute board position.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "move-node")]
pub struct MoveNode {
    pub node_id: String,
    pub new_x: f64,
    pub new_y: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn move_node(node_id: String, new_x: f64, new_y: f64) -> WiresMutation {
    WiresMutation::MoveNode(MoveNode { node_id, new_x, new_y })
}

impl protocol::MutationKind<WiresSnapshot, WiresMutation> for MoveNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "node", kind: "move-node", record: "MovedNode" };

    fn diff(&self, base: &WiresSnapshot) -> protocol::MutationOutcome<WiresDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &WiresSnapshot) -> Vec<WiresMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Move node \"{}\" to ({}, {})", self.node_id, self.new_x, self.new_y)
    }
    fn target(&self) -> Vec<String> {
        vec![self.node_id.clone()]
    }
}
//#endregion 🔖️Mutation
