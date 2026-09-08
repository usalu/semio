//! 🧩️ Set Contributions in the Playbook configuration channel.

use super::{PlaybookConfig, PlaybookConfigMutation, ReplaceConfig};

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-contributions")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetContributions {
    pub json: String,
}

impl protocol::MutationKind<PlaybookConfig, PlaybookConfigMutation> for SetContributions {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "contributions", kind: "set-contributions", record: "SetContributions" };
    fn diff(&self, base: &PlaybookConfig) -> protocol::MutationOutcome<PlaybookConfig> {
        let mut next = base.clone();
        next.contributions_json = self.json.clone();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &PlaybookConfig) -> Vec<PlaybookConfigMutation> { vec![PlaybookConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })] }
    fn label(&self) -> String { "Set Contributions".into() }
    fn target(&self) -> Vec<String> { vec!["contributions".into()] }
}
