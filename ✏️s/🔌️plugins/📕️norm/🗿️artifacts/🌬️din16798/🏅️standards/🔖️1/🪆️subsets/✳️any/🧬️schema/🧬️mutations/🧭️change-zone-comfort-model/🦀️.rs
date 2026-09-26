//! 🔧 `change-zone-comfort-model`.
use crate::{Din16798Mutation, Din16798Snapshot};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeZoneComfortModel {
    pub zone_id: String,
    pub new_comfort_model: String,
}
impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeZoneComfortModel {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "zone-comfort-model", kind: "change-zone-comfort-model", record: "ChangeZoneComfortModel" };
    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<<Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> { super::inverse::inverse(self, base) }
    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native("change-zone-comfort-model", "change-zone-comfort-model") }
    fn target(&self) -> Vec<String> { vec![self.zone_id.clone()] }
}
