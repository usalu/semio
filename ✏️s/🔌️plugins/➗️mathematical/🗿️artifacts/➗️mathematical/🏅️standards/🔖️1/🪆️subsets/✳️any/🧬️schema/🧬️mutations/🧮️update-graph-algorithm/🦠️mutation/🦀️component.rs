//! 🧮️ `update-graph-algorithm` — the algorithm id and its seed are validated together (the seed
//! is only meaningful in the context of the algorithm it seeds), so this is the recipe's
//! inseparable-facet `update` exception rather than two independent `change-` scalars — matches
//! the app's `SetAlgorithm` command, which always sends both fields together.

use crate::artifacts::mathematical::{MathematicalMutation, MathematicalSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateGraphAlgorithm {
    pub new_algorithm: String,
    pub new_algorithm_seed: Option<String>,
}

impl protocol::MutationKind<MathematicalSnapshot, MathematicalMutation> for UpdateGraphAlgorithm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "graph", kind: "update-graph-algorithm", record: "UpdatedGraphAlgorithm" };

    async fn diff(&self, base: &MathematicalSnapshot) -> protocol::MutationOutcome<<MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Set graph algorithm to \"{}\"", self.new_algorithm)
    }
    async fn target(&self) -> Vec<String> {
        vec!["graph".into(), "algorithm".into()]
    }
}
//#endregion 🔖️Payload
