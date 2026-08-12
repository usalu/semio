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

    fn diff(&self, base: &MathematicalSnapshot) -> <MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Replace graph".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["graph".into()]
    }
}
//#endregion 🔖️Payload
