//! 🧬️ The schema-owned Norm Viewport-window configuration: camera preset, hidden layers, colour map and unit system (persisted local-only per window).

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, dsl::DslArtifact, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact(id = "norm.viewport-window.config", extension = "normviewportcfg")]
#[dsl(layout = "lines")]
pub struct NormViewportWindowConfig {
    pub camera_preset: Option<String>,
    pub hidden_layers: Vec<String>,
    pub colour_map: Option<String>,
    pub unit_system: Option<String>,
}
