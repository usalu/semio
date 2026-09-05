//! 🧮️ `update-graph-algorithm` — the algorithm id and its seed are validated together (the seed
//! is only meaningful in the context of the algorithm it seeds), so this is the recipe's
//! inseparable-facet `update` exception rather than two independent `change-` scalars — matches
//! the app's `SetAlgorithm` command, which always sends both fields together.

use crate::artifacts::equation::{equation_children_from_state, equation_geometry, equation_graph, EquationDiff, EquationMutation, EquationSnapshot};
use semio_framework_os_kernel::{FromValue, ToValue};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct UpdateGraphAlgorithm {
    pub new_algorithm: String,
    pub new_algorithm_seed: Option<String>,
}

impl protocol::MutationKind<EquationSnapshot, EquationMutation> for UpdateGraphAlgorithm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "graph", kind: "update-graph-algorithm", record: "UpdatedGraphAlgorithm" };

    async fn diff(&self, base: &EquationSnapshot) -> protocol::MutationOutcome<<EquationMutation as protocol::Mutation<EquationSnapshot>>::Diff> {
        super::diff::diff(self, base).await
    }
    async fn inverse(&self, base: &EquationSnapshot) -> Vec<EquationMutation> {
        super::inverse::inverse(self, base).await
    }
    async fn label(&self) -> String {
        format!("Set graph algorithm to \"{}\"", self.new_algorithm)
    }
    async fn target(&self) -> Vec<String> {
        vec!["graph".into(), "algorithm".into()]
    }
}
//#endregion 🔖️Payload
