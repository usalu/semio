//! 🧬️ FEM 2D results window-config schema.

#[derive(Clone, Debug, PartialEq, dsl::DslArtifact, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(id = "fem.2d.resultswindowconfig", layout = "lines")]
pub struct Fem2dResultsWindowConfig {
    #[dsl(block)]
    pub camera: crate::Viewport2d,
    pub result_source_id: Option<String>,
    pub result_mode: crate::app_surface::ResultMode,
    pub result_mode_index: u32,
}
