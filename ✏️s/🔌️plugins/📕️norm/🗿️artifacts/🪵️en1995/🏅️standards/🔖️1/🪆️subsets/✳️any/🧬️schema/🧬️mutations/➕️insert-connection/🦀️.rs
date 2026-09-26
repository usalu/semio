//! ➕️ `insert-connection` — inserts one timber connection at an index (clamped to the list end).
use crate::{En1995Mutation, En1995Snapshot, TimberConnection};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertConnection { pub index: usize, pub connection: TimberConnection }
impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for InsertConnection {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "connection", kind: "insert-connection", record: "InsertedConnection" };
    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<<En1995Mutation as protocol::Mutation<En1995Snapshot>>::Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &En1995Snapshot) -> Vec<En1995Mutation> { super::inverse::inverse(self, base) }
    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native(&format!("Insert connection {} at #{}", self.connection.id, self.index), &format!("Verbindung {} an #{} einfügen", self.connection.id, self.index)) }
    fn target(&self) -> Vec<String> { vec![self.connection.id.clone()] }
}
