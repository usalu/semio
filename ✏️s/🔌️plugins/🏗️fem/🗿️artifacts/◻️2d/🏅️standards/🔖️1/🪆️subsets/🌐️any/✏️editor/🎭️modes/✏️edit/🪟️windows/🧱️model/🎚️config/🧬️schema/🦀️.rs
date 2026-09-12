//! 🧬️ FEM 2D model window-config schema.

#[derive(Clone, Debug, PartialEq, dsl::DslArtifact, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(id = "fem.2d.modelwindowconfig", layout = "lines")]
pub struct Fem2dModelWindowConfig {
    #[dsl(block)]
    pub camera: crate::Viewport2d,
}
