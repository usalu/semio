//! 🔧 `change-zone-occupants`.
use crate::{Din16798Mutation, Din16798Snapshot};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeZoneOccupants {
    pub zone_id: String,
    pub new_occupants: u32,
}
impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeZoneOccupants {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "zone-occupants", kind: "change-zone-occupants", record: "ChangeZoneOccupants" };
    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<<Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> { super::inverse::inverse(self, base) }
    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native("change-zone-occupants", "change-zone-occupants") }
    fn target(&self) -> Vec<String> { vec![self.zone_id.clone()] }
}
