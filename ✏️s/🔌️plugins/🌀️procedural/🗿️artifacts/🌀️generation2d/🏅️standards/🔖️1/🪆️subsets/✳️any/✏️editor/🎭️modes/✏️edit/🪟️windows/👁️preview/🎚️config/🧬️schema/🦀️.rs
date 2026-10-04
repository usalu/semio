//! 🧬️ Exact Generation2d edit-preview-window configuration schema.

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, semio_framework_os_kernel::DslArtifact, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(layout = "lines")]
#[artifact(id = "procedural.generation2d.editpreviewwindowconfig")]
pub struct Generation2dEditPreviewWindowConfig {
    #[dsl(block)]
    pub viewport: semio_framework_os_kernel::Viewport2d,
}
