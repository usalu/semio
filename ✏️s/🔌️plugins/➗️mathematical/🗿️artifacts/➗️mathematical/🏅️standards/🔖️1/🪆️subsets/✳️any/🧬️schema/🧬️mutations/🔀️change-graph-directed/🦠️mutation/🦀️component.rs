//! 🔀️ `change-graph-directed` — flips the graph playground's directed/undirected toggle.

use crate::artifacts::mathematical::{MathematicalMutation, MathematicalSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeGraphDirected {
    pub new_directed: bool,
}

impl protocol::MutationKind<MathematicalSnapshot, MathematicalMutation> for ChangeGraphDirected {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "graph", kind: "change-graph-directed", record: "ChangedGraphDirected" };

    async fn diff(&self, base: &MathematicalSnapshot) -> protocol::MutationOutcome<<MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Set graph direction to {}", if self.new_directed { "directed" } else { "undirected" })
    }
    async fn target(&self) -> Vec<String> {
        vec!["graph".into()]
    }
}
//#endregion 🔖️Payload
