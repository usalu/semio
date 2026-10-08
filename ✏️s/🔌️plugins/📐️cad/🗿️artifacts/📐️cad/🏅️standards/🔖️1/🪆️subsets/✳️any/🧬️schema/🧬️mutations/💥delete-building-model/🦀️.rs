//! 💥️ `delete-building-model` — clears the cad document's `building_model` CHILD slot. Idempotent
//! (a no-op if already empty); the inverse captures the escrowed handle from BASE so undo restores
//! it exactly.

use crate::mutations::CadMutation;
use crate::CadSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value_derive::RetireOwned)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-building-model")]
pub struct DeleteBuildingModel {}

impl MutationKind<CadSnapshot, CadMutation> for DeleteBuildingModel {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "building-model", kind: "delete-building-model", record: "DeletedBuildingModel" };

    fn diff(&self, base: &CadSnapshot) -> protocol::MutationOutcome<crate::diff::CadDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Result<Vec<CadMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Delete building-model child", "Gebäudemodell-Kind löschen")
    }
    fn target(&self) -> Vec<String> {
        vec!["building_model".to_string()]
    }
}
//#endregion 🔖️Mutation
