//! 🏢️ `create-building-model` — sets the cad document's `building_model` CHILD slot (composed
//! `s.stdio.semio.model`) to a new owned handle. If the slot was already occupied, this OVERWRITES
//! it (the inverse restores whichever handle was there before, not merely "delete" — see `↩️inverse`).

use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-building-model")]
pub struct CreateBuildingModel {
    pub child_id: String,
    /// 🔗️ The target's `ArtifactRef` flattened to its wire URI string (`to_uri()`) — `dsl::DslRecord`
    /// has no field-level support for `store::os_io::ArtifactRef` directly, mirrored at the diff
    /// boundary via `super::diff::parse_target`.
    pub target: String,
}

impl MutationKind<CadSnapshot, CadMutation> for CreateBuildingModel {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "building-model", kind: "create-building-model", record: "CreatedBuildingModel" };

    fn diff(&self, base: &CadSnapshot) -> crate::artifacts::cad::diff::CadDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Vec<CadMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create building-model child {}", self.child_id)
    }
    fn target(&self) -> Vec<String> {
        vec!["building_model".to_string()]
    }
}
//#endregion 🔖️Mutation
