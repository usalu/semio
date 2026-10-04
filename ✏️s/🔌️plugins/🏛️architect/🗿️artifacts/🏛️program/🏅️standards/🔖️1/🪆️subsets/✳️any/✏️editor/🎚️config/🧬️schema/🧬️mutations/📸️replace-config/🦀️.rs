//! 🧬️ Replace Config in the architect.config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "replace-config")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceConfig {
    #[dsl(block)]
    pub config: ArchitectConfig,
}

impl protocol::MutationKind<ArchitectConfig, ArchitectConfigMutation> for ReplaceConfig {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "config", kind: "replace-config", record: "ReplaceConfig" };
    fn diff(&self, base: &ArchitectConfig) -> protocol::MutationOutcome<ArchitectConfig> {
        if &self.config == base {
            return protocol::MutationOutcome::new(base.clone()).warning("mutation.no-op", "Requested config already matches.");
        }
        protocol::MutationOutcome::new(self.config.clone())
    }
    fn inverse(&self, base: &ArchitectConfig) -> Result<Vec<ArchitectConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![ArchitectConfigMutation::ReplaceConfig(Self { config: base.clone() })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace Config", "Konfiguration ersetzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["config".into()]
    }
}
