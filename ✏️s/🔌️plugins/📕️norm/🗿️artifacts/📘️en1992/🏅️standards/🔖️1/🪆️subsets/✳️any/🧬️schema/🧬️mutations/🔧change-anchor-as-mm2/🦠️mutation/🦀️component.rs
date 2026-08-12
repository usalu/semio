//! 🔧 `change-anchor-as-mm2` payload — changes the En1992 document's `anchor_a_s_mm2` (EN 1992 input).

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeAnchorASMm2
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAnchorASMm2 {
    pub new_anchor_a_s_mm2: f64,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ChangeAnchorASMm2 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "anchor-as-mm2", kind: "change-anchor-as-mm2", record: "ChangedAnchorASMm2" };

    fn diff(&self, base: &En1992Snapshot) -> En1992Diff {
        crate::artifacts::en1992::mutations::change_anchor_a_s_mm2::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1992Snapshot) -> Vec<En1992Mutation> {
        crate::artifacts::en1992::mutations::change_anchor_a_s_mm2::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change anchor a s mm2 to {:?}", self.new_anchor_a_s_mm2)
    }
}
//#endregion 🔖️ChangeAnchorASMm2
