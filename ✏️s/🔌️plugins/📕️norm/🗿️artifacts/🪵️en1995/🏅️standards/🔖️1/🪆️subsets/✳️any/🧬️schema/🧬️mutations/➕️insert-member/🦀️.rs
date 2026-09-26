//! ➕️ `insert-member` — inserts one timber member at an index (clamped to the list end).
use crate::{En1995Mutation, En1995Snapshot, TimberMember};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertMember { pub index: usize, pub member: TimberMember }
impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for InsertMember {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "member", kind: "insert-member", record: "InsertedMember" };
    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<<En1995Mutation as protocol::Mutation<En1995Snapshot>>::Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &En1995Snapshot) -> Vec<En1995Mutation> { super::inverse::inverse(self, base) }
    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native(&format!("Insert member {} at #{}", self.member.id, self.index), &format!("Bauteil {} an #{} einfügen", self.member.id, self.index)) }
    fn target(&self) -> Vec<String> { vec![self.member.id.clone()] }
}
