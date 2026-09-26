//! 🔧 `change-vent-heat-recovery`.
use crate::{Din16798Mutation, Din16798Snapshot};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeVentHeatRecovery {
    pub vent_id: String,
    pub new_heat_recovery_eta: f64,
}
impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeVentHeatRecovery {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "vent-heat-recovery", kind: "change-vent-heat-recovery", record: "ChangeVentHeatRecovery" };
    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<<Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> { super::inverse::inverse(self, base) }
    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native("change-vent-heat-recovery", "change-vent-heat-recovery") }
    fn target(&self) -> Vec<String> { vec![self.vent_id.clone()] }
}
