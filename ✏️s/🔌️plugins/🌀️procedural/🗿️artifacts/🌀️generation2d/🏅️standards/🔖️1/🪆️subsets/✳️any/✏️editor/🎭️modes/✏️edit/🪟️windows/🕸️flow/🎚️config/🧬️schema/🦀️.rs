//! 🧬️ Exact Generation2d main-window configuration schema.

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, semio_framework_os_kernel::DslArtifact, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(layout = "lines")]
#[artifact(id = "procedural.generation2d.mainwindowconfig")]
pub struct Generation2dMainWindowConfig {
    #[dsl(block)]
    pub viewport: semio_framework_os_kernel::Viewport2d,
}
