//! 🔧 `change-a-ef-mm2` payload — changes the En1995 document's `a_ef_mm2` (EN 1995 input).


use crate::artifacts::en1995::En1995Snapshot;
use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::mutations::change_a_ef_mm2::ChangeAEfMm2;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeAEfMm2
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAEfMm2 {
    pub new_a_ef_mm2: f64,
}

impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for ChangeAEfMm2 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "a-ef-mm2", kind: "change-a-ef-mm2", record: "ChangedAEfMm2" };

    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1995Snapshot) -> Vec<En1995Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change a ef mm2 to {:?}", self.new_a_ef_mm2)
    }
}
//#endregion 🔖️ChangeAEfMm2
