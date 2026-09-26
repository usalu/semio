//! 🔧 `change-vent-design-airflow`.
use crate::{Din16798Mutation, Din16798Snapshot};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeVentDesignAirflow {
    pub vent_id: String,
    pub new_design_airflow_m3_h: f64,
}
impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeVentDesignAirflow {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "vent-design-airflow", kind: "change-vent-design-airflow", record: "ChangeVentDesignAirflow" };
    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<<Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> { super::inverse::inverse(self, base) }
    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native("change-vent-design-airflow", "change-vent-design-airflow") }
    fn target(&self) -> Vec<String> { vec![self.vent_id.clone()] }
}
