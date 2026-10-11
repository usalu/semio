//! 📸️ Replace Config in the Forms configuration channel.

use super::*;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[dsl(keyword = "replace-config")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceConfig {
    #[dsl(block)]
    pub config: FormsConfig,
}

impl ReplaceConfig {
    /// 🧷️ The row that sets the config back to `base` — the config entity's replace-by-base value.
    fn restoring(base: &FormsConfig) -> FormsConfigMutation {
        FormsConfigMutation::ReplaceConfig(Self { config: base.clone() })
    }
}

impl protocol::MutationKind<FormsConfig, FormsConfigMutation> for ReplaceConfig {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "config", kind: "replace-config", record: "ReplaceConfig" };
    fn diff(&self, base: &FormsConfig) -> protocol::MutationOutcome<FormsConfigDiff> {
        protocol::MutationOutcome::new(FormsConfigDiff::changing(base, &self.config))
    }
    fn inverse(&self, base: &FormsConfig) -> Result<Vec<FormsConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self::restoring(base)]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace Config", "Konfiguration ersetzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["config".into()]
    }
}
