//! 🔧 `change-udl-kn-m` payload — changes the En1992 document's `udl_kn_m` (EN 1992 input).

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeUdlKnM
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeUdlKnM {
    pub new_udl_kn_m: f64,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ChangeUdlKnM {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "udl-kn-m", kind: "change-udl-kn-m", record: "ChangedUdlKnM" };

    async fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
        crate::artifacts::en1992::mutations::change_udl_kn_m::diff::diff(self, base)
    }

    async fn inverse(&self, base: &En1992Snapshot) -> Vec<En1992Mutation> {
        crate::artifacts::en1992::mutations::change_udl_kn_m::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change udl kn m to {:?}", self.new_udl_kn_m)
    }
}
//#endregion 🔖️ChangeUdlKnM
