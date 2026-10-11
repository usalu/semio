//! 🧬️ The shared schema-owned Norm Results-window configuration.

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, dsl::DslArtifact, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact(id = "norm.results-window.config", extension = "normresultscfg")]
#[dsl(layout = "lines")]
pub struct NormResultsWindowConfig {
    pub selected_check_index: Option<u32>,
}
