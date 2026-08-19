//! 🔌️ `delete-energy-model` — clears the cad document's `energy_model` CHILD slot. Idempotent
//! (a no-op if already empty); the inverse captures the escrowed handle from BASE so undo restores
//! it exactly.

use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-energy-model")]
pub struct DeleteEnergyModel {}

impl MutationKind<CadSnapshot, CadMutation> for DeleteEnergyModel {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "energy-model", kind: "delete-energy-model", record: "DeletedEnergyModel" };

    async fn diff(&self, base: &CadSnapshot) -> protocol::MutationOutcome<crate::artifacts::cad::diff::CadDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &CadSnapshot) -> Vec<CadMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "Delete energy-model child".to_string()
    }
    async fn target(&self) -> Vec<String> {
        vec!["energy_model".to_string()]
    }
}
//#endregion 🔖️Mutation
