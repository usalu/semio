//! 🌱 Wires mutation — `CreateNode`: brings one board node into existence (full initial payload,
//! id-keyed per `📓️derivation-rules.md` rule 2).

use crate::diff::WiresDiff;
use crate::mutations::WiresMutation;
use crate::schema::entity_id;
use crate::WiresSnapshot;
use dsl::DslValue;

//#region 🔖️Mutation
/// 🌱 `create-node` payload — the node's full initial state.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-node")]
pub struct CreateNode {
    pub node: DslValue,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant, in the DSL's canonical key order
/// (`crate::canonical_board_value`): the op-text printer sorts object keys while `DslValue`'s
/// `PartialEq` compares entries positionally, so an unsorted payload cannot survive
/// `parse_op(print_op(op)) == op`.
pub fn create_node(node: DslValue) -> WiresMutation {
    WiresMutation::CreateNode(CreateNode { node: crate::canonical_board_value(&node) })
}

impl protocol::MutationKind<WiresSnapshot, WiresMutation> for CreateNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "node", kind: "create-node", record: "CreatedNode" };

    fn diff(&self, base: &WiresSnapshot) -> protocol::MutationOutcome<WiresDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &WiresSnapshot) -> Vec<WiresMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Add node \"{}\"", entity_id(&self.node, "id").unwrap_or("?")), &format!("Knoten \"{}\" hinzufügen", entity_id(&self.node, "id").unwrap_or("?")))
    }
    fn target(&self) -> Vec<String> {
        entity_id(&self.node, "id").map(|id| vec![id.to_string()]).unwrap_or_default()
    }
}
//#endregion 🔖️Mutation
