//! 🔧 `change-zone-co2`.
use crate::{Din16798Mutation, Din16798Snapshot};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeZoneCo2 {
    pub zone_id: String,
    pub new_co2_ppm: f64,
}
impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeZoneCo2 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "zone-co2", kind: "change-zone-co2", record: "ChangeZoneCo2" };
    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<<Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> { super::inverse::inverse(self, base) }
    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native("change-zone-co2", "change-zone-co2") }
    fn target(&self) -> Vec<String> { vec![self.zone_id.clone()] }
}
