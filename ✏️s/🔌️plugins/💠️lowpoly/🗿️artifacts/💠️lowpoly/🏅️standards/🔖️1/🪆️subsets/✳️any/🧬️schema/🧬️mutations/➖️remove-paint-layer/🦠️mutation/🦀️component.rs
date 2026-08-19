//! ➖️ `remove-paint-layer` — takes a paint layer out of an object's ordered (compositing-order)
//! layer list at a BASE-state index. Reuses this directory's pre-existing path (glue.rs still
//! `#[path]`-wires it) — same kebab slug survives the semantic-mutations rewrite unchanged.

use crate::artifacts::lowpoly::{LowpolyMutation, LowpolySnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemovePaintLayer {
    pub object_id: String,
    pub index: usize,
}

impl protocol::MutationKind<LowpolySnapshot, LowpolyMutation> for RemovePaintLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "paint-layer", kind: "remove-paint-layer", record: "RemovedPaintLayer" };

    async fn diff(&self, base: &LowpolySnapshot) -> protocol::MutationOutcome<<LowpolyMutation as protocol::Mutation<LowpolySnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &LowpolySnapshot) -> Vec<LowpolyMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Remove paint layer {} from object \"{}\"", self.index, self.object_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.object_id.clone()]
    }
}
//#endregion 🔖️Payload
