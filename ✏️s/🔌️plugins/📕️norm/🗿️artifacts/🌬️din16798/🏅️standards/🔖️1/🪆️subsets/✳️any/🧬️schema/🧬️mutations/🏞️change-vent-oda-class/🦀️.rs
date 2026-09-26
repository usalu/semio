//! 🔧 `change-vent-oda-class`.
use crate::{Din16798Mutation, Din16798Snapshot};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeVentOdaClass {
    pub vent_id: String,
    pub new_oda_class: String,
}
impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeVentOdaClass {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "vent-oda-class", kind: "change-vent-oda-class", record: "ChangeVentOdaClass" };
    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<<Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> { super::inverse::inverse(self, base) }
    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native("change-vent-oda-class", "change-vent-oda-class") }
    fn target(&self) -> Vec<String> { vec![self.vent_id.clone()] }
}
