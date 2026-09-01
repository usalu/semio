//! 🔧 `change-d-mm` payload — changes the En1992 document's `d_mm` (EN 1992 input).


use crate::artifacts::en1992::En1992Snapshot;
use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::mutations::change_d_mm::ChangeDMm;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeDMm
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeDMm {
    pub new_d_mm: f64,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ChangeDMm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "d-mm", kind: "change-d-mm", record: "ChangedDMm" };

    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1992Snapshot) -> Vec<En1992Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change d mm to {:?}", self.new_d_mm)
    }
}
//#endregion 🔖️ChangeDMm
