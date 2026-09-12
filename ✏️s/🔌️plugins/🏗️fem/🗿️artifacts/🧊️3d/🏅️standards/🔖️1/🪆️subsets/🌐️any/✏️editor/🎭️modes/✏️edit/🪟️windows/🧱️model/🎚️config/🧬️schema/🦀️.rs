//! 🧬️ FEM 3D model window-config schema.

#[derive(Clone, Debug, PartialEq, dsl::DslArtifact, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(id = "fem.3d.modelwindowconfig", layout = "lines")]
pub struct Fem3dModelWindowConfig {
    #[dsl(block)]
    pub camera: crate::Viewport3dOrbit,
}
