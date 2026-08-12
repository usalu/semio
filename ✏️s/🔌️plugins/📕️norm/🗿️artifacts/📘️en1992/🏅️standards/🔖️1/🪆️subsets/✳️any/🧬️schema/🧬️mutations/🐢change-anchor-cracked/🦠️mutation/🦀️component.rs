//! 🔧 `change-anchor-cracked` payload — changes the En1992 document's `anchor_cracked` (EN 1992 input).

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeAnchorCracked
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAnchorCracked {
    pub new_anchor_cracked: bool,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ChangeAnchorCracked {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "anchor-cracked", kind: "change-anchor-cracked", record: "ChangedAnchorCracked" };

    fn diff(&self, base: &En1992Snapshot) -> En1992Diff {
        crate::artifacts::en1992::mutations::change_anchor_cracked::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1992Snapshot) -> Vec<En1992Mutation> {
        crate::artifacts::en1992::mutations::change_anchor_cracked::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change anchor cracked to {:?}", self.new_anchor_cracked)
    }
}
//#endregion 🔖️ChangeAnchorCracked
