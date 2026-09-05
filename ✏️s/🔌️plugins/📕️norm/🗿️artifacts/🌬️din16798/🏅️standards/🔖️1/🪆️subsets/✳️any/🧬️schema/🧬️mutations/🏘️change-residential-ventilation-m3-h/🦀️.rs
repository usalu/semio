//! 🔧 `change-residential-ventilation-m3-h` payload — changes the Din16798 document's `residential_ventilation_m3_h` (residential ventilation air flow).


use crate::artifacts::din16798::Din16798Snapshot;
use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
//#region 🔖️ChangeResidentialVentilationM3H
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeResidentialVentilationM3H {
    pub new_residential_ventilation_m3_h: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeResidentialVentilationM3H {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "residential-ventilation-m3-h", kind: "change-residential-ventilation-m3-h", record: "ChangedResidentialVentilationM3H" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change residential ventilation air flow to {}", self.new_residential_ventilation_m3_h)
    }
}
//#endregion 🔖️ChangeResidentialVentilationM3H
