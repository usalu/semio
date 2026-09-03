//! ✂️ `disconnect-nodes` — removes an edge relationship between two graph nodes.

use crate::artifacts::equation::standards::v1::subsets::graph::schema::mutations::connect_nodes;
use crate::artifacts::equation::{equation_children_from_state, equation_geometry, equation_graph, EquationDiff, EquationMutation, EquationSnapshot};
use semio_framework_os_kernel::{FromValue, ToValue};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct DisconnectNodes {
    pub id: String,
}

impl protocol::MutationKind<EquationSnapshot, EquationMutation> for DisconnectNodes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "disconnect", entity: "node", kind: "disconnect-nodes", record: "DisconnectedNodes" };

    async fn diff(&self, base: &EquationSnapshot) -> protocol::MutationOutcome<<EquationMutation as protocol::Mutation<EquationSnapshot>>::Diff> {
        super::diff::diff(self, base).await
    }
    async fn inverse(&self, base: &EquationSnapshot) -> Vec<EquationMutation> {
        super::inverse::inverse(self, base).await
    }
    async fn label(&self) -> String {
        format!("Disconnect edge \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
