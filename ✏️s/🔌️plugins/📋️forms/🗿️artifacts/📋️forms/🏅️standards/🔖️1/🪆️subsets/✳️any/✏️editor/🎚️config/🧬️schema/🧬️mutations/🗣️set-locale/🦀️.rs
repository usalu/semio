//! 🗣️ Set Locale in the Forms configuration channel.

use super::*;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-locale")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetLocale {
    pub value: String,
}

impl protocol::MutationKind<FormsConfig, FormsConfigMutation> for SetLocale {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "locale", kind: "set-locale", record: "SetLocale" };
    fn diff(&self, base: &FormsConfig) -> protocol::MutationOutcome<FormsConfig> {
        let mut next = base.clone();
        next.locale = self.value.clone();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &FormsConfig) -> Vec<FormsConfigMutation> { vec![FormsConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })] }
    fn label(&self) -> String { "Set Locale".into() }
    fn target(&self) -> Vec<String> { vec!["locale".into()] }
}
