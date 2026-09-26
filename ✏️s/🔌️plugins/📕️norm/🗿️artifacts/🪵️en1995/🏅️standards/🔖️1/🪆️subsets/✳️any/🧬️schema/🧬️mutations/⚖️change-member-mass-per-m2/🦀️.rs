//! ⚖️ `change-member-mass-per-m2` — changes `massKgPerM2` (Area mass) on one addressed member.
use crate::{En1995Mutation, En1995Snapshot};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeMemberMassPerM2 { pub member_id: String, pub new_value: f64 }
impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for ChangeMemberMassPerM2 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "member-massKgPerM2", kind: "change-member-mass-per-m2", record: "ChangedMemberMassPerM2" };
    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<<En1995Mutation as protocol::Mutation<En1995Snapshot>>::Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &En1995Snapshot) -> Vec<En1995Mutation> { super::inverse::inverse(self, base) }
    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native(&format!("Change Area mass of member {}", self.member_id), &format!("Flächenbezogene Masse von Bauteil {} ändern", self.member_id)) }
    fn target(&self) -> Vec<String> { vec![self.member_id.clone()] }
}
