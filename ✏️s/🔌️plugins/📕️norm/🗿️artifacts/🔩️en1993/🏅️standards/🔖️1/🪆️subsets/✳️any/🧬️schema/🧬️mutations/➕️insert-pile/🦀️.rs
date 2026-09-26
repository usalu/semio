//! ➕️ insert-pile
use crate::{SteelPile, En1993Mutation, En1993Snapshot};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertPile { pub index: usize, pub pile: SteelPile }
impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for InsertPile {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "pile", kind: "insert-pile", record: "InsertedPile" };
    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> { super::inverse::inverse(self, base) }
    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native(&format!("Insert pile at #{}", self.index), &format!("pile an #{} einfügen", self.index)) }
    fn target(&self) -> Vec<String> { vec![self.index.to_string()] }
}
