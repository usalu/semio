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

    fn diff(&self, base: &MathematicalSnapshot) -> protocol::MutationOutcome<<MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Set graph direction to {}", if self.new_directed { "directed" } else { "undirected" })
    }
    fn target(&self) -> Vec<String> {
        vec!["graph".into()]
    }
}
//#endregion 🔖️Payload
