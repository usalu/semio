//! 📍️ Wires mutation — `SetNodePositions`: puts a set of board nodes at absolute positions in ONE row. It is the exact undo
//! of a `move-nodes` drag (every moved node back at its base position) and of itself, so a multi-node gesture stays one
//! point-invertible row however many nodes it moved.

use crate::diff::WiresDiff;
use crate::mutations::WiresMutation;
use crate::WiresSnapshot;

//#region 🔖️Mutation
/// 📌️ One node's absolute board position.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct WiresNodePosition {
    pub node_id: String,
    pub x: f64,
    pub y: f64,
}

/// 📍️ `set-node-positions` payload — FINAL-state absolute positions, each node once.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "set-node-positions")]
pub struct SetNodePositions {
    pub positions: Vec<WiresNodePosition>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn set_node_positions(positions: Vec<WiresNodePosition>) -> WiresMutation {
    WiresMutation::SetNodePositions(SetNodePositions { positions })
}

impl SetNodePositions {
    /// 🆔️ The positioned node ids, in payload order.
    pub fn node_ids(&self) -> Vec<String> {
        self.positions.iter().map(|position| position.node_id.clone()).collect()
    }
}

impl protocol::MutationKind<WiresSnapshot, WiresMutation> for SetNodePositions {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "node-positions", kind: "set-node-positions", record: "SetNodePositions" };

    fn diff(&self, base: &WiresSnapshot) -> protocol::MutationOutcome<WiresDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &WiresSnapshot) -> Vec<WiresMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set the positions of {} node(s)", self.positions.len()), &format!("Positionen von {} Knoten setzen", self.positions.len()))
    }
    fn target(&self) -> Vec<String> {
        self.node_ids()
    }
}
//#endregion 🔖️Mutation
