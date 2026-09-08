//! 🧬️ Replace Presence in the Writer presence channel.

use super::*;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "replace-presence")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplacePresence {
    #[dsl(block)]
    pub presence: WriterPresence,
}

impl protocol::MutationKind<WriterPresence, WriterPresenceMutation> for ReplacePresence {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "presence", kind: "replace-presence", record: "ReplacedPresence" };
    fn diff(&self, _base: &WriterPresence) -> protocol::MutationOutcome<WriterPresence> {
        protocol::MutationOutcome::new(self.presence.clone())
    }
    fn inverse(&self, base: &WriterPresence) -> Vec<WriterPresenceMutation> { vec![WriterPresenceMutation::ReplacePresence(ReplacePresence { presence: base.clone() })] }
    fn label(&self) -> String { "Replace Presence".into() }
    fn target(&self) -> Vec<String> { vec!["presence".into()] }
}
