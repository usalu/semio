//! 🗣️ SetLocale changes only the addressed configuration field.

use super::{EquationConfig, EquationConfigMutation};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[dsl(keyword = "locale")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetLocale {
    pub value: String,
}

impl protocol::MutationKind<EquationConfig, EquationConfigMutation> for SetLocale {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "locale", kind: "set-locale", record: "SetLocale" };
    fn diff(&self, base: &EquationConfig) -> protocol::MutationOutcome<EquationConfig> {
        if base.locale == self.value { return protocol::MutationOutcome::new(base.clone()).warn("mutation.no-op", "Configuration field is unchanged."); }
        protocol::MutationOutcome::new(EquationConfig { locale: self.value.clone(), ..base.clone() })
    }
    fn inverse(&self, base: &EquationConfig) -> Vec<EquationConfigMutation> {
        if base.locale == self.value { Vec::new() } else { vec![EquationConfigMutation::SetLocale(SetLocale { value: base.locale.clone() })] }
    }
    fn label(&self) -> String { "Set Locale".into() }
    fn target(&self) -> Vec<String> { vec!["locale".into()] }
}
