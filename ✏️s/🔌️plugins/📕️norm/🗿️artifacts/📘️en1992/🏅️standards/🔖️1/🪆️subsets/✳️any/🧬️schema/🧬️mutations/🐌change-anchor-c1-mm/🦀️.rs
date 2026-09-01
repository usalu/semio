//! 🔧 `change-anchor-c1-mm` payload — changes the En1992 document's `anchor_c1_mm` (EN 1992 input).


use crate::artifacts::en1992::En1992Snapshot;
use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::mutations::change_anchor_c1_mm::ChangeAnchorC1Mm;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeAnchorC1Mm
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAnchorC1Mm {
    pub new_anchor_c1_mm: f64,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ChangeAnchorC1Mm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "anchor-c1-mm", kind: "change-anchor-c1-mm", record: "ChangedAnchorC1Mm" };

    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1992Snapshot) -> Vec<En1992Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change anchor c1 mm to {:?}", self.new_anchor_c1_mm)
    }
}
//#endregion 🔖️ChangeAnchorC1Mm
