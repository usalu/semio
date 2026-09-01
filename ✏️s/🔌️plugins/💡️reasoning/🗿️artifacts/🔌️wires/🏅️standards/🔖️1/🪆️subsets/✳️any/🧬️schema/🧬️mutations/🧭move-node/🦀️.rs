//! 🧭️ Wires mutation — `MoveNode`: absolute spatial reposition of one board node
//! (`📓️taxonomy.md`'s `move` verb). Replaces the old generic `PatchNode{x,y}` call sites
//! (force-layout, canvas drag).

use crate::artifacts::wires::diff::{diff_board_fixture, WiresDiff};
use crate::artifacts::wires::mutations::{set_node_field, WiresMutation};
use crate::artifacts::wires::schema::node_position;
use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::artifacts::wires::WiresSnapshot;
use dsl::DslValue;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🧭️ `move-node` payload — the node's new absolute board position.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "move-node")]
pub struct MoveNode {
    pub node_id: String,
    pub new_x: f64,
    pub new_y: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn move_node(node_id: String, new_x: f64, new_y: f64) -> WiresMutation {
    WiresMutation::MoveNode(MoveNode { node_id, new_x, new_y })
}

impl protocol::MutationKind<WiresSnapshot, WiresMutation> for MoveNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "node", kind: "move-node", record: "MovedNode" };

    async fn diff(&self, base: &WiresSnapshot) -> protocol::MutationOutcome<WiresDiff> {
        super::diff::diff(self, base).await
    }
    async fn inverse(&self, base: &WiresSnapshot) -> Vec<WiresMutation> {
        super::inverse::inverse(self, base).await
    }
    async fn label(&self) -> String {
        format!("Move node \"{}\" to ({}, {})", self.node_id, self.new_x, self.new_y)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.node_id.clone()]
    }
}
//#endregion 🔖️Mutation
