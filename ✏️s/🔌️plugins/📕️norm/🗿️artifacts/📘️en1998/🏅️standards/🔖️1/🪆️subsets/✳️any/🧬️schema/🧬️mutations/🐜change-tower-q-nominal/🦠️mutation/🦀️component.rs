//! 🐜 `change-tower-q-nominal` payload — changes the En1998 document's `tower_q_nominal` (tower nominal behaviour factor q).

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeTowerQNominal
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeTowerQNominal {
    pub new_tower_q_nominal: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeTowerQNominal {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "tower-q-nominal", kind: "change-tower-q-nominal", record: "ChangedTowerQNominal" };

    async fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        crate::artifacts::en1998::mutations::change_tower_q_nominal::diff::diff(self, base)
    }

    async fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        crate::artifacts::en1998::mutations::change_tower_q_nominal::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change tower nominal behaviour factor q to {}", self.new_tower_q_nominal)
    }
}
//#endregion 🔖️ChangeTowerQNominal
