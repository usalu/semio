//! 🧬️ Exact Generation2d generate-preview-window configuration schema.

#[derive(Clone, Debug, PartialEq, dsl::DslArtifact, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(id = "procedural.generation2d.generatepreviewwindowconfig", layout = "lines")]
pub struct Generation2dGeneratePreviewWindowConfig {
    #[dsl(block)]
    pub viewport: semio_framework_os_kernel::Viewport2d,
}
