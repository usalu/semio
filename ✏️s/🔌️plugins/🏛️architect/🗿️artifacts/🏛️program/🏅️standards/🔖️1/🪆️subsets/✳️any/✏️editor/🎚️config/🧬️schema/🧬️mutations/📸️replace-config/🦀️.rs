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
    fn diff(&self, base: &ArchitectConfig) -> protocol::MutationOutcome<ArchitectConfigDiff> {
        if &self.config == base {
            return protocol::MutationOutcome::new(ArchitectConfigDiff::default()).warning("mutation.no-op", "Requested config already matches.");
        }
        let differing = |current: &String, requested: &String| (current != requested).then(|| requested.clone());
        protocol::MutationOutcome::new(ArchitectConfigDiff {
            search_query: differing(&base.search_query, &self.config.search_query),
            search_history_json: differing(&base.search_history_json, &self.config.search_history_json),
            last_result_json: differing(&base.last_result_json, &self.config.last_result_json),
            last_analysis_json: differing(&base.last_analysis_json, &self.config.last_analysis_json),
        })
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
