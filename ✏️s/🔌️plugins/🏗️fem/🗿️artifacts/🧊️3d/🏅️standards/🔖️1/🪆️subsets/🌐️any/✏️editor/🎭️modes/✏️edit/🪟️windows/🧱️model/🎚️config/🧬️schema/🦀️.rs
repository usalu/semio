//! 🧬️ FEM 3D model window-config schema.

/// 🧭️ Which handles the transform gumball of ONE model window draws — view state, toggled from the
/// Transform utility's options rail (`setTransformGumballFlag`), never a document field.
#[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::DslRecord, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct Fem3dGumballConfig {
    pub move_axes: bool,
    pub move_planes: bool,
    pub rotate: bool,
    pub scale_axes: bool,
    pub scale_uniform: bool,
}

#[derive(Clone, Debug, PartialEq, dsl::DslArtifact, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(id = "fem.3d.modelwindowconfig", layout = "lines")]
pub struct Fem3dModelWindowConfig {
    #[dsl(block)]
    pub camera: crate::Viewport3dOrbit,
    #[dsl(block)]
    pub gumball: Fem3dGumballConfig,
}
