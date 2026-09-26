//! ➕️ `insert-member-action` — inserts one characteristic action into a member's action table.
use crate::{CharacteristicAction, En1995Mutation, En1995Snapshot};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertMemberAction { pub member_id: String, pub index: usize, pub action: CharacteristicAction }
impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for InsertMemberAction {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "member-action", kind: "insert-member-action", record: "InsertedMemberAction" };
    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<<En1995Mutation as protocol::Mutation<En1995Snapshot>>::Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &En1995Snapshot) -> Vec<En1995Mutation> { super::inverse::inverse(self, base) }
    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native(&format!("Insert action {} into member {} at #{}", self.action.id, self.member_id, self.index), &format!("Einwirkung {} in Bauteil {} an #{} einfügen", self.action.id, self.member_id, self.index)) }
    fn target(&self) -> Vec<String> { vec![self.member_id.clone(), self.action.id.clone()] }
}
