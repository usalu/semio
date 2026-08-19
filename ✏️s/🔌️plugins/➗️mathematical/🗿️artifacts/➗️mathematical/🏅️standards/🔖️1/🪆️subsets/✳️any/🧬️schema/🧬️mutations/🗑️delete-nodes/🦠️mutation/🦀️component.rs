//! 🗑️ `delete-nodes` — plural/bulk delete, the real multi-select gesture behind the node-graph
//! canvas's `deleteSelection` edit op (`✏️editor/🎮️commands/🕸️set-algorithm/component.rs`) —
//! a separate mutation per taxonomy's "Bulk/plural mutations" rule, never a bare `Vec` bolted onto
//! the singular `delete-node`.

use crate::artifacts::mathematical::{MathematicalMutation, MathematicalSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteNodes {
    pub ids: Vec<String>,
}

impl protocol::MutationKind<MathematicalSnapshot, MathematicalMutation> for DeleteNodes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "nodes", kind: "delete-nodes", record: "DeletedNodes" };

    async fn diff(&self, base: &MathematicalSnapshot) -> protocol::MutationOutcome<<MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete {} nodes", self.ids.len())
    }
    async fn target(&self) -> Vec<String> {
        self.ids.clone()
    }
}
//#endregion 🔖️Payload
