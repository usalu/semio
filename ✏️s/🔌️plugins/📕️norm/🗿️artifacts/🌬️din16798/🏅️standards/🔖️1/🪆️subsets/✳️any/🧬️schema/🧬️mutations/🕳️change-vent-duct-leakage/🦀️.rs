//! 🔧 `change-vent-duct-leakage`.
use crate::{Din16798Mutation, Din16798Snapshot};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeVentDuctLeakage {
    pub vent_id: String,
    pub new_duct_leakage_m3_s_m2: f64,
}
impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeVentDuctLeakage {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "vent-duct-leakage", kind: "change-vent-duct-leakage", record: "ChangeVentDuctLeakage" };
    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<<Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> { super::inverse::inverse(self, base) }
    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native("change-vent-duct-leakage", "change-vent-duct-leakage") }
    fn target(&self) -> Vec<String> { vec![self.vent_id.clone()] }
}
