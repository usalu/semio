//! 🌴 `change-qb-kpa` payload — changes the En1997 document's `q_b_kpa` (base resistance q_b [kPa]).

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::En1997Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeQBKpa
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeQBKpa {
    pub new_q_b_kpa: f64,
}

impl protocol::MutationKind<En1997Snapshot, En1997Mutation> for ChangeQBKpa {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "qb-kpa", kind: "change-qb-kpa", record: "ChangedQBKpa" };

    fn diff(&self, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
        crate::artifacts::en1997::mutations::change_q_b_kpa::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1997Snapshot) -> Vec<En1997Mutation> {
        crate::artifacts::en1997::mutations::change_q_b_kpa::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change base resistance q_b [kPa] to {}", self.new_q_b_kpa)
    }
}
//#endregion 🔖️ChangeQBKpa
