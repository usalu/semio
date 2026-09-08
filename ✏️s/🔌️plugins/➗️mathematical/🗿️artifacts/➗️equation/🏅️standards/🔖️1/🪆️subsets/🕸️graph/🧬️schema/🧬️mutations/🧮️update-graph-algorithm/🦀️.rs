//! 🧮️ `update-graph-algorithm` — the algorithm id and its seed are validated together (the seed
//! is only meaningful in the context of the algorithm it seeds), so this is the recipe's
//! inseparable-facet `update` exception rather than two independent `change-` scalars — matches
//! the app's `SetAlgorithm` command, which always sends both fields together.

use crate::{EquationMutation, EquationSnapshot};
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

    fn diff(&self, base: &EquationSnapshot) -> protocol::MutationOutcome<<EquationMutation as protocol::Mutation<EquationSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &EquationSnapshot) -> Vec<EquationMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Set graph algorithm to \"{}\"", self.new_algorithm)
    }
    fn target(&self) -> Vec<String> {
        vec!["graph".into(), "algorithm".into()]
    }
}
//#endregion 🔖️Payload
