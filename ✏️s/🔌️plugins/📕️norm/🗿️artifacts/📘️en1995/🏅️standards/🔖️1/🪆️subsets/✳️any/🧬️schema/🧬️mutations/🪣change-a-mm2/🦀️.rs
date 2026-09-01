//! 🔧 `change-a-mm2` payload — changes the En1995 document's `a_mm2` (EN 1995 input).


use crate::artifacts::en1995::En1995Snapshot;
use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::mutations::change_a_mm2::ChangeAMm2;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeAMm2
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAMm2 {
    pub new_a_mm2: f64,
}

impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for ChangeAMm2 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "a-mm2", kind: "change-a-mm2", record: "ChangedAMm2" };

    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1995Snapshot) -> Vec<En1995Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change a mm2 to {:?}", self.new_a_mm2)
    }
}
//#endregion 🔖️ChangeAMm2
