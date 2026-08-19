//! 🔁️ `replace-graph` — whole-value swap of the graph playground's structured payload (nodes,
//! edges, algorithm, direction all at once) — the semantic replacement for the old generic
//! `SetGraph`, used by gestures that load/paste an entire graph (e.g. the app's `SetArtifact`
//! command) rather than editing one field or one node/edge.

use crate::artifacts::mathematical::{MathematicalGraph, MathematicalMutation, MathematicalSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplaceGraph {
    pub graph: MathematicalGraph,
}

impl protocol::MutationKind<MathematicalSnapshot, MathematicalMutation> for ReplaceGraph {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "graph", kind: "replace-graph", record: "ReplacedGraph" };

    async fn diff(&self, base: &MathematicalSnapshot) -> protocol::MutationOutcome<<MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "Replace graph".into()
    }
    async fn target(&self) -> Vec<String> {
        vec!["graph".into()]
    }
}
//#endregion 🔖️Payload
