//! 🔧 `change-span-m` payload — changes the En1992 document's `span_m` (EN 1992 input).


use crate::artifacts::en1992::En1992Snapshot;
use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::mutations::change_span_m::ChangeSpanM;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeSpanM
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeSpanM {
    pub new_span_m: f64,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ChangeSpanM {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "span-m", kind: "change-span-m", record: "ChangedSpanM" };

    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1992Snapshot) -> Vec<En1992Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change span m to {:?}", self.new_span_m)
    }
}
//#endregion 🔖️ChangeSpanM
