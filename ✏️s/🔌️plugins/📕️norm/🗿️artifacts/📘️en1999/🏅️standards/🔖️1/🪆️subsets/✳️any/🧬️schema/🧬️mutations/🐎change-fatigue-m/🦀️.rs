//! 🐎 `change-fatigue-m` payload — changes the En1999 document's `fatigue_m` (fatigue S-N slope m).


use crate::artifacts::en1999::En1999Snapshot;
use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::mutations::change_fatigue_m::ChangeFatigueM;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeFatigueM
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeFatigueM {
    pub new_fatigue_m: f64,
}

impl protocol::MutationKind<En1999Snapshot, En1999Mutation> for ChangeFatigueM {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fatigue-m", kind: "change-fatigue-m", record: "ChangedFatigueM" };

    fn diff(&self, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1999Snapshot) -> Vec<En1999Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change fatigue S-N slope m to {}", self.new_fatigue_m)
    }
}
//#endregion 🔖️ChangeFatigueM
