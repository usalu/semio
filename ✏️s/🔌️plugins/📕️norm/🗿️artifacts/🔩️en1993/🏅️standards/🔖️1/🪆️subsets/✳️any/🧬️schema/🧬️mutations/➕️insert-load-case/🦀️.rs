//! ➕️ insert-load-case
use crate::{LoadCase, En1993Mutation, En1993Snapshot};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertLoadCase { pub index: usize, pub load_case: LoadCase, }
impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for InsertLoadCase {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "load-case", kind: "insert-load-case", record: "InsertedLoadCase" };
    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> { super::inverse::inverse(self, base) }
    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native(&format!("Insert load-case at #{}", self.index), &format!("load-case an #{} einfügen", self.index)) }
    fn target(&self) -> Vec<String> { vec![self.index.to_string()] }
}
