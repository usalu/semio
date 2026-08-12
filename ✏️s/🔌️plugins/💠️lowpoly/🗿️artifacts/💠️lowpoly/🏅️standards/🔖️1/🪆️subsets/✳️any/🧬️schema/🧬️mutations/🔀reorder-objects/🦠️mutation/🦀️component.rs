//! 🔀️ `reorder-objects` — repositions an object within the document's ordered object list (display
//! order in the outliner; never spatial — see `move-object`/`rotate-object`/`scale-object` for that).

use crate::artifacts::lowpoly::{LowpolyMutation, LowpolySnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReorderObjects {
    pub id: String,
    pub to_index: usize,
}

impl protocol::MutationKind<LowpolySnapshot, LowpolyMutation> for ReorderObjects {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "object", kind: "reorder-objects", record: "ReorderedObjects" };

    fn diff(&self, base: &LowpolySnapshot) -> <LowpolyMutation as protocol::Mutation<LowpolySnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &LowpolySnapshot) -> Vec<LowpolyMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Reorder object \"{}\" to {}", self.id, self.to_index)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
