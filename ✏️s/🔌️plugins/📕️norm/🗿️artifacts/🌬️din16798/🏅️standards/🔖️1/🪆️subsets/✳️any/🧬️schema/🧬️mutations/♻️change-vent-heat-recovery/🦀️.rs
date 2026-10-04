//! 🔧 `change-vent-heat-recovery`.
use crate::{Din16798Mutation, Din16798Snapshot};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeVentHeatRecovery {
    pub vent_id: String,
    pub new_heat_recovery_eta: f64,
}
impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeVentHeatRecovery {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "vent-heat-recovery", kind: "change-vent-heat-recovery", record: "ChangeVentHeatRecovery" };
    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<<Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &Din16798Snapshot) -> Result<Vec<Din16798Mutation>, semio_framework_value::ValueError> {
    Ok({ super::inverse::inverse(self, base)? 
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change heat recovery efficiency", "Wärmerückgewinnungsgrad ändern")
    }
    fn target(&self) -> Vec<String> { vec![self.vent_id.clone()] }
}
