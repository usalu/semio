//! 🔌️ `delete-energy-model` — clears the cad document's `energy_model` CHILD slot. Idempotent
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
#[dsl(keyword = "delete-energy-model")]
pub struct DeleteEnergyModel {}

impl MutationKind<CadSnapshot, CadMutation> for DeleteEnergyModel {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "energy-model", kind: "delete-energy-model", record: "DeletedEnergyModel" };

    fn diff(&self, base: &CadSnapshot) -> protocol::MutationOutcome<crate::diff::CadDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Result<Vec<CadMutation>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Delete energy-model child", "Energiemodell-Kind löschen")
    }
    fn target(&self) -> Vec<String> {
        vec!["energy_model".to_string()]
    }
}
//#endregion 🔖️Mutation
