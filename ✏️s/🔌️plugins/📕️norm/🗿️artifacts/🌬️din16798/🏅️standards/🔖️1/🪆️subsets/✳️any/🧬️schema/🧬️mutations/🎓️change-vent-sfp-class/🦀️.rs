//! 🔧 `change-vent-sfp-class`.
use crate::{Din16798Mutation, Din16798Snapshot};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeVentSfpClass {
    pub vent_id: String,
    pub new_sfp_required_class: u8,
}
impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeVentSfpClass {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "vent-sfp-class", kind: "change-vent-sfp-class", record: "ChangeVentSfpClass" };
    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<<Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> { super::inverse::inverse(self, base) }
    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native("change-vent-sfp-class", "change-vent-sfp-class") }
    fn target(&self) -> Vec<String> { vec![self.vent_id.clone()] }
}
