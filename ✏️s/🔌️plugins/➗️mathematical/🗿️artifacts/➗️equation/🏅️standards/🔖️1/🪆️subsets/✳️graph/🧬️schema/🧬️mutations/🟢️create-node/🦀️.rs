//! 🟢️ `create-node` — brings a new id-keyed graph node into existence.

use crate::artifacts::equation::standards::v1::subsets::graph::schema::mutations::delete_node;
use crate::artifacts::equation::{equation_children_from_state, equation_geometry, equation_graph, EquationDiff, EquationMutation, EquationNode, EquationSnapshot};
use semio_framework_os_kernel::{FromValue, ToValue};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateNode {
    pub id: String,
    pub label: String,
    pub x: f64,
    pub y: f64,
}

impl protocol::MutationKind<EquationSnapshot, EquationMutation> for CreateNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "node", kind: "create-node", record: "CreatedNode" };

    async fn diff(&self, base: &EquationSnapshot) -> protocol::MutationOutcome<<EquationMutation as protocol::Mutation<EquationSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &EquationSnapshot) -> Vec<EquationMutation> {
        super::inverse::inverse(self, base).await
    }
    async fn label(&self) -> String {
        format!("Create node \"{}\"", self.label)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
