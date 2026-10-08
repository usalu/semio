//! 🔄️ Replace Config in the WFC 3D config facet — the whole-record swap a host restore performs.

use super::{Wfc3dConfig, Wfc3dConfigDiff, Wfc3dConfigMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "replace-config")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceConfig {
    #[dsl(block)]
    pub config: Wfc3dConfig,
}

impl protocol::MutationKind<Wfc3dConfig, Wfc3dConfigMutation> for ReplaceConfig {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "config", kind: "replace-config", record: "ReplaceConfig" };
    fn diff(&self, base: &Wfc3dConfig) -> protocol::MutationOutcome<Wfc3dConfigDiff> {
        protocol::MutationOutcome::new(<Wfc3dConfigDiff as protocol::DiffAlgebra<Wfc3dConfig>>::between(base, &self.config))
    }
    fn inverse(&self, base: &Wfc3dConfig) -> Result<Vec<Wfc3dConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Wfc3dConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace Config", "Konfiguration ersetzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["config".into()]
    }
}
