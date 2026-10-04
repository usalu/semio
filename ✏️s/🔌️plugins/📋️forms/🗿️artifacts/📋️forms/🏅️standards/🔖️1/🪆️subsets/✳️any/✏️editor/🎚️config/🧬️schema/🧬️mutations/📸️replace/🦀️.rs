//! 📸️ Replace Config in the Forms configuration channel.

use super::*;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "replace-config")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceConfig {
    #[dsl(block)]
    pub config: FormsConfig,
}

impl protocol::MutationKind<FormsConfig, FormsConfigMutation> for ReplaceConfig {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "config", kind: "replace-config", record: "ReplaceConfig" };
    fn diff(&self, _base: &FormsConfig) -> protocol::MutationOutcome<FormsConfig> {
        protocol::MutationOutcome::new(self.config.clone())
    }
    fn inverse(&self, base: &FormsConfig) -> Result<Vec<FormsConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![FormsConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace Config", "Konfiguration ersetzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["config".into()]
    }
}
