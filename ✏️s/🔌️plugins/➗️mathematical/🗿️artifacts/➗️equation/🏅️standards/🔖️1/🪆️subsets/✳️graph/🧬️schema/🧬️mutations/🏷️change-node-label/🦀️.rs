//! 🏷️ `change-node-label` — sets a node's display label (`id` stays the stable identity field, so
//! this is `change`, not `rename`).

use crate::artifacts::equation::{equation_children_from_state, equation_geometry, equation_graph, EquationDiff, EquationMutation, EquationSnapshot};
use semio_framework_os_kernel::{FromValue, ToValue};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeNodeLabel {
    pub id: String,
    pub new_label: String,
}

impl protocol::MutationKind<EquationSnapshot, EquationMutation> for ChangeNodeLabel {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node", kind: "change-node-label", record: "ChangedNodeLabel" };

    async fn diff(&self, base: &EquationSnapshot) -> protocol::MutationOutcome<<EquationMutation as protocol::Mutation<EquationSnapshot>>::Diff> {
        super::diff::diff(self, base).await
    }
    async fn inverse(&self, base: &EquationSnapshot) -> Vec<EquationMutation> {
        super::inverse::inverse(self, base).await
    }
    async fn label(&self) -> String {
        format!("Relabel node \"{}\" to \"{}\"", self.id, self.new_label)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
