//! 💥️ `delete-building-model` — clears the cad document's `building_model` CHILD slot. Idempotent
//! (a no-op if already empty); the inverse captures the escrowed handle from BASE so undo restores
//! it exactly.

use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value_derive::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-building-model")]
pub struct DeleteBuildingModel {}

impl<Op> MutationKind<CadSnapshot, Op> for DeleteBuildingModel
where
    Op: protocol::Mutation<CadSnapshot, Diff = semio_s_artifact_cad_cad_model_diff::CadDiff> + From<crate::create_building_model::CreateBuildingModel>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "building-model", kind: "delete-building-model", record: "DeletedBuildingModel" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Delete building-model child", "Gebäudemodell-Kind löschen")
    }
    fn target(&self) -> Vec<String> {
        vec!["building_model".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<CadSnapshot, Op> for DeleteBuildingModel
where
    Op: protocol::ColdMutation<CadSnapshot, ReferenceDiff = semio_s_artifact_cad_cad_model_diff::CadDiff> + From<crate::create_building_model::CreateBuildingModel>,
{
    fn diff(&self, base: &CadSnapshot) -> protocol::MutationOutcome<semio_s_artifact_cad_cad_model_diff::CadDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
}
//#endregion 🔖️Mutation
