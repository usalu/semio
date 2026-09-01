//! 🐴 `change-delta-sigma-c` payload — changes the En1999 document's `delta_sigma_c` (fatigue reference stress range [MPa]).


use crate::artifacts::en1999::En1999Snapshot;
use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::mutations::change_delta_sigma_c::ChangeDeltaSigmaC;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeDeltaSigmaC
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeDeltaSigmaC {
    pub new_delta_sigma_c: f64,
}

impl protocol::MutationKind<En1999Snapshot, En1999Mutation> for ChangeDeltaSigmaC {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "delta-sigma-c", kind: "change-delta-sigma-c", record: "ChangedDeltaSigmaC" };

    fn diff(&self, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1999Snapshot) -> Vec<En1999Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change fatigue reference stress range [MPa] to {}", self.new_delta_sigma_c)
    }
}
//#endregion 🔖️ChangeDeltaSigmaC
