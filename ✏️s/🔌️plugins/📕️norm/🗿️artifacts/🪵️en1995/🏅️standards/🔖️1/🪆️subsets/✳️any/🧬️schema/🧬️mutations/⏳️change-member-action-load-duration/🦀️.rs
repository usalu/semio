//! ⏳️ `change-member-action-load-duration` — changes `loadDuration` (Load-duration class) on one addressed member.
use crate::{En1995Mutation, En1995Snapshot};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeMemberActionLoadDuration { pub member_id: String, pub action_id: String, pub new_value: String }
impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for ChangeMemberActionLoadDuration {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "member-action-loadDuration", kind: "change-member-action-load-duration", record: "ChangedMemberActionLoadDuration" };
    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<<En1995Mutation as protocol::Mutation<En1995Snapshot>>::Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &En1995Snapshot) -> Vec<En1995Mutation> { super::inverse::inverse(self, base) }
    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native(&format!("Change Load-duration class of action {} on member {}", self.action_id, self.member_id), &format!("Klasse der Lasteinwirkungsdauer der Einwirkung {} an Bauteil {} ändern", self.action_id, self.member_id)) }
    fn target(&self) -> Vec<String> { vec![self.member_id.clone(), self.action_id.clone()] }
}
