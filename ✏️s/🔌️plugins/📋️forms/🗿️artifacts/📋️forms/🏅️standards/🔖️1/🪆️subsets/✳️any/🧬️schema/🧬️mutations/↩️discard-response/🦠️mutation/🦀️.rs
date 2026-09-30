//! 📨️ DiscardResponse persists one semantic response event.
use crate::{FormMutation, FormsDiff, FormsSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DiscardResponse {
    pub id: String,
}

impl MutationKind<FormsSnapshot, FormMutation> for DiscardResponse {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "discard", entity: "response", kind: "discard-response", record: "DiscardedResponse" };
    fn diff(&self, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &FormsSnapshot) -> Vec<FormMutation> { super::inverse::inverse(self, base) }
    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native("Retract Response", "Antwort zurückziehen") }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}
