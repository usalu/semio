//! 📸️ Replace Config in the Playbook configuration channel.

use super::{PlaybookConfig, PlaybookConfigDiff, PlaybookConfigMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "replace-config")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceConfig {
    #[dsl(block)]
    pub config: PlaybookConfig,
}

impl protocol::MutationKind<PlaybookConfig, PlaybookConfigMutation> for ReplaceConfig {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "config", kind: "replace-config", record: "ReplaceConfig" };
    fn diff(&self, base: &PlaybookConfig) -> protocol::MutationOutcome<PlaybookConfigDiff> {
        protocol::MutationOutcome::new(PlaybookConfigDiff { contributions_json: (base.contributions_json != self.config.contributions_json).then(|| self.config.contributions_json.clone()) })
    }
    fn inverse(&self, base: &PlaybookConfig) -> Result<Vec<PlaybookConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![PlaybookConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace Config", "Konfiguration ersetzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["config".into()]
    }
}

#[cfg(test)]
mod law_tests {
    use super::*;

    /// ⚖️ The inverse diffs sum to the negative of the forward diff (L3).
    #[semio_framework_async_macros::async_test]
    async fn inverse_diffs_sum_to_the_negative_diff() {
        let base = PlaybookConfig::default();
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&PlaybookConfigMutation::ReplaceConfig(ReplaceConfig { config: PlaybookConfig { contributions_json: "[1]".into() } }), &base).await;
    }
}
