//! 🔧 `change-f-ed-kn` payload — changes the En1995 document's `f_ed_kn` (EN 1995 input).


use crate::artifacts::en1995::En1995Snapshot;
use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::mutations::change_f_ed_kn::ChangeFEdKn;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeFEdKn
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeFEdKn {
    pub new_f_ed_kn: f64,
}

impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for ChangeFEdKn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "f-ed-kn", kind: "change-f-ed-kn", record: "ChangedFEdKn" };

    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1995Snapshot) -> Vec<En1995Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change f ed kn to {:?}", self.new_f_ed_kn)
    }
}
//#endregion 🔖️ChangeFEdKn
