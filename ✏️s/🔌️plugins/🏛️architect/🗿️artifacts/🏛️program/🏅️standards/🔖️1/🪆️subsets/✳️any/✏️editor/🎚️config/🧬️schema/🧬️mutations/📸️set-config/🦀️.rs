//! 🧬️ Set Config in the architect.config channel.

use super::*;

/// 📸️ Sets the named config fields to absolute values; a field left `None` stays untouched.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-config")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetConfig {
    pub search_query: Option<String>,
    pub search_history_json: Option<String>,
    pub last_result_json: Option<String>,
    pub last_analysis_json: Option<String>,
}

impl SetConfig {
    fn changes(&self, base: &ArchitectConfig) -> ArchitectConfigDiff {
        let differing = |current: &String, requested: &Option<String>| requested.as_ref().filter(|value| *value != current).cloned();
        ArchitectConfigDiff {
            search_query: differing(&base.search_query, &self.search_query),
            search_history_json: differing(&base.search_history_json, &self.search_history_json),
            last_result_json: differing(&base.last_result_json, &self.last_result_json),
            last_analysis_json: differing(&base.last_analysis_json, &self.last_analysis_json),
        }
    }
}

impl protocol::MutationKind<ArchitectConfig, ArchitectConfigMutation> for SetConfig {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "config", kind: "set-config", record: "SetConfig" };
    fn diff(&self, base: &ArchitectConfig) -> protocol::MutationOutcome<ArchitectConfigDiff> {
        let changes = self.changes(base);
        if changes == ArchitectConfigDiff::default() {
            return protocol::MutationOutcome::new(changes).warning("mutation.no-op", "Requested config fields already match.");
        }
        protocol::MutationOutcome::new(changes)
    }
    fn inverse(&self, base: &ArchitectConfig) -> Result<Vec<ArchitectConfigMutation>, semio_framework_value::ValueError> {
        let changes = self.changes(base);
        Ok(vec![ArchitectConfigMutation::SetConfig(Self {
            search_query: changes.search_query.map(|_| base.search_query.clone()),
            search_history_json: changes.search_history_json.map(|_| base.search_history_json.clone()),
            last_result_json: changes.last_result_json.map(|_| base.last_result_json.clone()),
            last_analysis_json: changes.last_analysis_json.map(|_| base.last_analysis_json.clone()),
        })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Config", "Konfiguration setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["config".into()]
    }
}
