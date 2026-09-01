//! 🔧 `change-f-ck` payload — changes the En1992 document's `f_ck` (EN 1992 input).


use crate::artifacts::en1992::En1992Snapshot;
use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::mutations::change_f_ck::ChangeFCk;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeFCk
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeFCk {
    pub new_f_ck: f64,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ChangeFCk {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "f-ck", kind: "change-f-ck", record: "ChangedFCk" };

    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1992Snapshot) -> Vec<En1992Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change f ck to {:?}", self.new_f_ck)
    }
}
//#endregion 🔖️ChangeFCk
