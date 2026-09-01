//! 🕷️ `change-chi` payload — changes the En1999 document's `chi` (buckling reduction factor chi).


use crate::artifacts::en1999::En1999Snapshot;
use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::mutations::change_chi::ChangeChi;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeChi
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeChi {
    pub new_chi: f64,
}

impl protocol::MutationKind<En1999Snapshot, En1999Mutation> for ChangeChi {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "chi", kind: "change-chi", record: "ChangedChi" };

    fn diff(&self, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1999Snapshot) -> Vec<En1999Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change buckling reduction factor chi to {}", self.new_chi)
    }
}
//#endregion 🔖️ChangeChi
