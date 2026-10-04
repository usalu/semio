//! 🧬️ FEM 2D model window-config schema.

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, semio_framework_os_kernel::DslArtifact, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(layout = "lines")]
#[artifact(id = "fem.2d.modelwindowconfig")]
pub struct Fem2dModelWindowConfig {
    #[dsl(block)]
    pub camera: crate::Viewport2d,
}
