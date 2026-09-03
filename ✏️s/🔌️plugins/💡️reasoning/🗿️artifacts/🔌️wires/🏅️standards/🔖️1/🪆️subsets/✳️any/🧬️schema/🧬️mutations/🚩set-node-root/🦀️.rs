//! 🚩 Wires mutation — `SetNodeRoot`: narrow addressed single-field boolean setter for one board
//! node's `root` flag (`📓️taxonomy.md`'s `set` verb — the exact `set-layer-visible` shape).

use crate::artifacts::wires::diff::{diff_board_fixture, WiresDiff};
use crate::artifacts::wires::mutations::{set_node_field, WiresMutation};
use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::artifacts::wires::WiresSnapshot;
use dsl::DslValue;

//#region 🔖️Mutation
/// 🚩 `set-node-root` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "set-node-root")]
pub struct SetNodeRoot {
    pub node_id: String,
    pub new_root: bool,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn set_node_root(node_id: String, new_root: bool) -> WiresMutation {
    WiresMutation::SetNodeRoot(SetNodeRoot { node_id, new_root })
}

impl protocol::MutationKind<WiresSnapshot, WiresMutation> for SetNodeRoot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "node", kind: "set-node-root", record: "SetNodeRoot" };

    async fn diff(&self, base: &WiresSnapshot) -> protocol::MutationOutcome<WiresDiff> {
        super::diff::diff(self, base).await
    }
    async fn inverse(&self, base: &WiresSnapshot) -> Vec<WiresMutation> {
        super::inverse::inverse(self, base).await
    }
    async fn label(&self) -> String {
        format!("Set node \"{}\" root to {}", self.node_id, self.new_root)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.node_id.clone()]
    }
}
//#endregion 🔖️Mutation
