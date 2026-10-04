//! 🧬️ FEM 3D results window-config schema.

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, semio_framework_os_kernel::DslArtifact, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(layout = "lines")]
#[artifact(id = "fem.3d.resultswindowconfig")]
pub struct Fem3dResultsWindowConfig {
    #[dsl(block)]
    pub camera: crate::Viewport3dOrbit,
    pub result_source_id: Option<String>,
    pub result_mode: crate::app_surface::ResultMode,
    pub result_mode_index: u32,
    #[dsl(block)]
    pub animation: crate::app_surface::FemResultsAnimation,
}
