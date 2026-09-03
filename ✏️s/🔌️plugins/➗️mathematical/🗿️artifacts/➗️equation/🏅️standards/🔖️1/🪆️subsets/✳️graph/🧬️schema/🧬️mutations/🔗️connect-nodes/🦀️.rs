//! 🔗️ `connect-nodes` — creates an edge relationship between two graph nodes (the node-graph
//! canvas's `connect` edit op).

use crate::artifacts::equation::standards::v1::subsets::graph::schema::mutations::disconnect_nodes;
use crate::artifacts::equation::{equation_children_from_state, equation_geometry, equation_graph, EquationDiff, EquationEdge, EquationMutation, EquationSnapshot};
use semio_framework_os_kernel::{FromValue, ToValue};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ConnectNodes {
    pub id: String,
    pub source: String,
    pub target: String,
}

impl protocol::MutationKind<EquationSnapshot, EquationMutation> for ConnectNodes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "connect", entity: "node", kind: "connect-nodes", record: "ConnectedNodes" };

    async fn diff(&self, base: &EquationSnapshot) -> protocol::MutationOutcome<<EquationMutation as protocol::Mutation<EquationSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &EquationSnapshot) -> Vec<EquationMutation> {
        super::inverse::inverse(self, base).await
    }
    async fn label(&self) -> String {
        format!("Connect \"{}\" to \"{}\"", self.source, self.target)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
