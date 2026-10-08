//! 🔄️ Replace Config in the WFC 2D config facet.

use super::{Wfc2dConfig, Wfc2dConfigDiff, Wfc2dConfigMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", deny_unknown_fields))]
#[dsl(keyword = "replace-config")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceConfig {
    #[dsl(block)]
    pub config: Wfc2dConfig,
}

impl protocol::MutationKind<Wfc2dConfig, Wfc2dConfigMutation> for ReplaceConfig {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "config", kind: "replace-config", record: "ReplaceConfig" };
    fn diff(&self, base: &Wfc2dConfig) -> protocol::MutationOutcome<Wfc2dConfigDiff> {
        protocol::MutationOutcome::new(<Wfc2dConfigDiff as protocol::DiffAlgebra<Wfc2dConfig>>::between(base, &self.config))
    }
    fn inverse(&self, base: &Wfc2dConfig) -> Result<Vec<Wfc2dConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Wfc2dConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace Config", "Konfiguration ersetzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["config".into()]
    }
}
