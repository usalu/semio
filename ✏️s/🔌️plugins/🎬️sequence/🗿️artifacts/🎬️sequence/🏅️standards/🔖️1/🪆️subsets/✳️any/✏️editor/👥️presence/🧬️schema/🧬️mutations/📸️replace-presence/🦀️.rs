//! 🧬️ Replace Presence in the sequence.presence channel.

use super::*;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "replace-presence")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplacePresence {
    #[dsl(block)]
    pub presence: SequencePresence,
}

impl protocol::MutationKind<SequencePresence, SequencePresenceMutation> for ReplacePresence {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "presence", kind: "replace-presence", record: "ReplacePresence" };
    fn diff(&self, _base: &SequencePresence) -> protocol::MutationOutcome<SequencePresence> {
        protocol::MutationOutcome::new(self.presence.clone())
    }
    fn inverse(&self, base: &SequencePresence) -> Vec<SequencePresenceMutation> { vec![SequencePresenceMutation::ReplacePresence(Self { presence: base.clone() })] }
    fn label(&self) -> String { "Replace Presence".into() }
    fn target(&self) -> Vec<String> { vec!["presence".into()] }
}
