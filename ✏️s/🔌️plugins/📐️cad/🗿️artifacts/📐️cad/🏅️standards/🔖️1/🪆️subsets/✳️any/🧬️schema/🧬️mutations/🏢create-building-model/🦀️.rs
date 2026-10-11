//! 🏢️ `create-building-model` — sets the cad document's `building_model` CHILD slot (composed
//! `s.stdio.semio.model`) to a new owned handle. If the slot was already occupied, this OVERWRITES
//! it (the inverse restores whichever handle was there before, not merely "delete" — see `↩️inverse`).

use crate::mutations::CadMutation;
use crate::CadSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value_derive::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-building-model")]
pub struct CreateBuildingModel {
    pub child_id: String,
    /// 🪪️ Exact composed child identity.
    pub target: semio_framework_artifact_reference::ArtifactRef,
}

impl MutationKind<CadSnapshot, CadMutation> for CreateBuildingModel {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "building-model", kind: "create-building-model", record: "CreatedBuildingModel" };

    fn diff(&self, base: &CadSnapshot) -> protocol::MutationOutcome<crate::diff::CadDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Result<Vec<CadMutation>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create building-model child {}", self.child_id), &format!("Gebäudemodell-Kind {} erstellen", self.child_id))
    }
    fn target(&self) -> Vec<String> {
        vec!["building_model".to_string()]
    }
}
//#endregion 🔖️Mutation
