//! ⚡️ `create-energy-model` — sets the cad document's `energy_model` CHILD slot (composed
//! `s.stdio.semio.model`) to a new owned handle. If the slot was already occupied, this OVERWRITES
//! it (the inverse restores whichever handle was there before, not merely "delete" — see `↩️inverse`).

use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-energy-model")]
pub struct CreateEnergyModel {
    pub child_id: String,
    /// 🔗️ The target's `ArtifactRef` flattened to its wire URI string (`to_uri()`) — `dsl::DslRecord`
    /// has no field-level support for `store::os_io::ArtifactRef` directly, mirrored at the diff
    /// boundary via `super::diff::parse_target`.
    pub target: String,
}

impl MutationKind<CadSnapshot, CadMutation> for CreateEnergyModel {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "energy-model", kind: "create-energy-model", record: "CreatedEnergyModel" };

    async fn diff(&self, base: &CadSnapshot) -> protocol::MutationOutcome<crate::artifacts::cad::diff::CadDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &CadSnapshot) -> Vec<CadMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create energy-model child {}", self.child_id)
    }
    async fn target(&self) -> Vec<String> {
        vec!["energy_model".to_string()]
    }
}
//#endregion 🔖️Mutation
