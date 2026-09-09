//! 🧬️ Replace Presence in the architect.presence channel.

use super::*;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "replace-presence")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplacePresence {
    #[dsl(block)]
    pub presence: ArchitectPresence,
}

impl protocol::MutationKind<ArchitectPresence, ArchitectPresenceMutation> for ReplacePresence {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "presence", kind: "replace-presence", record: "ReplacePresence" };
    fn diff(&self, base: &ArchitectPresence) -> protocol::MutationOutcome<ArchitectPresence> {
        if &self.presence == base {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", "Requested presence already matches.");
        }
        protocol::MutationOutcome::new(self.presence.clone())
    }
    fn inverse(&self, base: &ArchitectPresence) -> Vec<ArchitectPresenceMutation> {
        vec![ArchitectPresenceMutation::ReplacePresence(Self { presence: base.clone() })]
    }
    fn label(&self) -> String {
        "Replace Presence".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["presence".into()]
    }
}
