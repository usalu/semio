//! 🚚️ Wires mutation — `MoveNodes`: one canvas node drag as intent (design §13.3): the board offset every addressed node
//! moves by from its BASE position, so editing the drag in history replays it on whatever base it lands on.

use crate::diff::WiresDiff;
use crate::mutations::WiresMutation;
use crate::WiresSnapshot;

//#region 🔖️Mutation
/// 🚚️ `move-nodes` payload — RELATIVE `(dx, dy)` over the addressed nodes.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "move-nodes")]
pub struct MoveNodes {
    pub node_ids: Vec<String>,
    pub dx: f64,
    pub dy: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn move_nodes(node_ids: Vec<String>, dx: f64, dy: f64) -> WiresMutation {
    WiresMutation::MoveNodes(MoveNodes { node_ids, dx, dy })
}

impl protocol::MutationKind<WiresSnapshot, WiresMutation> for MoveNodes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "nodes", kind: "move-nodes", record: "MovedNodes" };

    fn diff(&self, base: &WiresSnapshot) -> protocol::MutationOutcome<WiresDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &WiresSnapshot) -> Vec<WiresMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move {} node(s) by ({}, {})", self.node_ids.len(), self.dx, self.dy), &format!("{} Knoten um ({}; {}) verschieben", self.node_ids.len(), self.dx, self.dy))
    }
    fn target(&self) -> Vec<String> {
        self.node_ids.clone()
    }
}
//#endregion 🔖️Mutation
