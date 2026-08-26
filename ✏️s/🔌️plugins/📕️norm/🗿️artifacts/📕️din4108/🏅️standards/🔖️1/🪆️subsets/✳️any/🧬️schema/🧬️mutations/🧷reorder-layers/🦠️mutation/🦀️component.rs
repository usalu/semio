//! 🔀️ `reorder-layers` — repositions one construction layer within the build-up order (never
//! spatial — `LayerDocument` carries no position of its own, only build-up sequence).

use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReorderLayers {
    pub from: usize,
    pub to: usize,
}

impl protocol::MutationKind<Din4108Snapshot, Din4108Mutation> for ReorderLayers {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "layers", kind: "reorder-layers", record: "ReorderedLayers" };

    fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Move layer #{} to #{}", self.from, self.to)
    }
    fn target(&self) -> Vec<String> {
        vec![self.from.to_string()]
    }
}
//#endregion 🔖️Payload
