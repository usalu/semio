//! ⚖️ `change-connection-action-kind` — changes `kind` (Action kind) on one addressed connection.
use crate::{En1995Mutation, En1995Snapshot};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeConnectionActionKind { pub connection_id: String, pub action_id: String, pub new_value: String }
impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for ChangeConnectionActionKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "connection-action-kind", kind: "change-connection-action-kind", record: "ChangedConnectionActionKind" };
    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<<En1995Mutation as protocol::Mutation<En1995Snapshot>>::Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &En1995Snapshot) -> Vec<En1995Mutation> { super::inverse::inverse(self, base) }
    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native(&format!("Change Action kind of action {} on connection {}", self.action_id, self.connection_id), &format!("Einwirkungsart der Einwirkung {} an Verbindung {} ändern", self.action_id, self.connection_id)) }
    fn target(&self) -> Vec<String> { vec![self.connection_id.clone(), self.action_id.clone()] }
}
