//! 🐷 `change-beta-w` payload — changes the En1999 document's `beta_w` (correlation factor beta_w).


use crate::artifacts::en1999::En1999Snapshot;
use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::mutations::change_beta_w::ChangeBetaW;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeBetaW
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeBetaW {
    pub new_beta_w: f64,
}

impl protocol::MutationKind<En1999Snapshot, En1999Mutation> for ChangeBetaW {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "beta-w", kind: "change-beta-w", record: "ChangedBetaW" };

    fn diff(&self, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1999Snapshot) -> Vec<En1999Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change correlation factor beta_w to {}", self.new_beta_w)
    }
}
//#endregion 🔖️ChangeBetaW
