//! ➕️ insert-tension-component
use crate::{TensionComponent, En1993Mutation, En1993Snapshot};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertTensionComponent { pub index: usize, pub tension_component: TensionComponent }
impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for InsertTensionComponent {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "tension-component", kind: "insert-tension-component", record: "InsertedTensionComponent" };
    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> { super::inverse::inverse(self, base) }
    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native(&format!("Insert tension-component at #{}", self.index), &format!("tension-component an #{} einfügen", self.index)) }
    fn target(&self) -> Vec<String> { vec![self.index.to_string()] }
}
