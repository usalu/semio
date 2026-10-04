//! 🧬️ FEM 3D model window-config schema.

/// 🧭️ Which handles the transform gumball of ONE model window draws — view state, toggled from the
/// Transform utility's options rail (`setTransformGumballFlag`), never a document field.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslRecord, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct Fem3dGumballConfig {
    pub move_axes: bool,
    pub move_planes: bool,
    pub rotate: bool,
    pub scale_axes: bool,
    pub scale_uniform: bool,
}

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, semio_framework_os_kernel::DslArtifact, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(layout = "lines")]
#[artifact(id = "fem.3d.modelwindowconfig")]
pub struct Fem3dModelWindowConfig {
    #[dsl(block)]
    pub camera: crate::Viewport3dOrbit,
    #[dsl(block)]
    pub gumball: Fem3dGumballConfig,
}
