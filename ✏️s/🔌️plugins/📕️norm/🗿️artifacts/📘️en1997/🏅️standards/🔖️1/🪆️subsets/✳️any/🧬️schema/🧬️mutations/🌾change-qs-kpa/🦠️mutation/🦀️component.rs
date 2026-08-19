//! 🌾 `change-qs-kpa` payload — changes the En1997 document's `q_s_kpa` (shaft resistance q_s [kPa]).

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::En1997Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeQSKpa
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeQSKpa {
    pub new_q_s_kpa: f64,
}

impl protocol::MutationKind<En1997Snapshot, En1997Mutation> for ChangeQSKpa {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "qs-kpa", kind: "change-qs-kpa", record: "ChangedQSKpa" };

    async fn diff(&self, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
        crate::artifacts::en1997::mutations::change_q_s_kpa::diff::diff(self, base)
    }

    async fn inverse(&self, base: &En1997Snapshot) -> Vec<En1997Mutation> {
        crate::artifacts::en1997::mutations::change_q_s_kpa::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change shaft resistance q_s [kPa] to {}", self.new_q_s_kpa)
    }
}
//#endregion 🔖️ChangeQSKpa
