//! 🐎 `change-fatigue-m` payload — changes the En1999 document's `fatigue_m` (fatigue S-N slope m).

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeFatigueM
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeFatigueM {
    pub new_fatigue_m: f64,
}

impl protocol::MutationKind<En1999Snapshot, En1999Mutation> for ChangeFatigueM {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fatigue-m", kind: "change-fatigue-m", record: "ChangedFatigueM" };

    async fn diff(&self, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
        crate::artifacts::en1999::mutations::change_fatigue_m::diff::diff(self, base)
    }

    async fn inverse(&self, base: &En1999Snapshot) -> Vec<En1999Mutation> {
        crate::artifacts::en1999::mutations::change_fatigue_m::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change fatigue S-N slope m to {}", self.new_fatigue_m)
    }
}
//#endregion 🔖️ChangeFatigueM
