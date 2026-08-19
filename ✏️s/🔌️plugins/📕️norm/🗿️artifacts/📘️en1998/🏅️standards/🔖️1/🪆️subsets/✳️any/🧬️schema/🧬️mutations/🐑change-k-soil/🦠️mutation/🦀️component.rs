//! 🐑 `change-k-soil` payload — changes the En1998 document's `k_soil` (soil stiffness k [kN/m]).

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeKSoil
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeKSoil {
    pub new_k_soil: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeKSoil {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "k-soil", kind: "change-k-soil", record: "ChangedKSoil" };

    async fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        crate::artifacts::en1998::mutations::change_k_soil::diff::diff(self, base)
    }

    async fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        crate::artifacts::en1998::mutations::change_k_soil::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change soil stiffness k [kN/m] to {}", self.new_k_soil)
    }
}
//#endregion 🔖️ChangeKSoil
