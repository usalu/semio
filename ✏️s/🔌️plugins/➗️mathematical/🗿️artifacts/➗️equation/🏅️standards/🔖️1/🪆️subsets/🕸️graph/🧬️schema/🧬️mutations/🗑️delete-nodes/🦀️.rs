//! 🗑️ `delete-nodes` — plural/bulk delete, the real multi-select gesture behind the node-graph
//! canvas's `deleteSelection` edit op (`✏️editor/🎮️commands/🧮️set-algorithm/component.rs`) —
//! a separate mutation per taxonomy's "Bulk/plural mutations" rule, never a bare `Vec` bolted onto
//! the singular `delete-node`.

use crate::artifacts::equation::mutations::{connect_nodes, create_node};
use crate::artifacts::equation::{equation_children_from_state, equation_geometry, equation_graph, EquationDiff, EquationMutation, EquationSnapshot};
use semio_framework_os_kernel::{FromValue, ToValue};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteNodes {
    pub ids: Vec<String>,
}

impl protocol::MutationKind<EquationSnapshot, EquationMutation> for DeleteNodes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "nodes", kind: "delete-nodes", record: "DeletedNodes" };

    async fn diff(&self, base: &EquationSnapshot) -> protocol::MutationOutcome<<EquationMutation as protocol::Mutation<EquationSnapshot>>::Diff> {
        super::diff::diff(self, base).await
    }
    async fn inverse(&self, base: &EquationSnapshot) -> Vec<EquationMutation> {
        super::inverse::inverse(self, base).await
    }
    async fn label(&self) -> String {
        format!("Delete {} nodes", self.ids.len())
    }
    async fn target(&self) -> Vec<String> {
        self.ids.clone()
    }
}
//#endregion 🔖️Payload
