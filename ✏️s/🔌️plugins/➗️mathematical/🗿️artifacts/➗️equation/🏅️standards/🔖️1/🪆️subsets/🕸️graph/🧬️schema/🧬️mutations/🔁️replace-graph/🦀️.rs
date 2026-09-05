//! 🔁️ `replace-graph` — whole-value swap of the graph playground's structured payload (nodes,
//! edges, algorithm, direction all at once) — the semantic replacement for the old generic
//! `SetGraph`, used by gestures that load/paste an entire graph (e.g. the app's `SetArtifact`
//! command) rather than editing one field or one node/edge.

use crate::artifacts::equation::{equation_children_from_state, equation_geometry, equation_graph, EquationDiff, EquationGraph, EquationMutation, EquationSnapshot};
use semio_framework_os_kernel::{FromValue, ToValue};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceGraph {
    pub graph: EquationGraph,
}

impl protocol::MutationKind<EquationSnapshot, EquationMutation> for ReplaceGraph {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "graph", kind: "replace-graph", record: "ReplacedGraph" };

    async fn diff(&self, base: &EquationSnapshot) -> protocol::MutationOutcome<<EquationMutation as protocol::Mutation<EquationSnapshot>>::Diff> {
        super::diff::diff(self, base).await
    }
    async fn inverse(&self, base: &EquationSnapshot) -> Vec<EquationMutation> {
        super::inverse::inverse(self, base).await
    }
    async fn label(&self) -> String {
        "Replace graph".into()
    }
    async fn target(&self) -> Vec<String> {
        vec!["graph".into()]
    }
}
//#endregion 🔖️Payload
