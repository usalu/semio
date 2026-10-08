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

/// 🩹 Owned-field patch of [`Fem3dModelWindowConfig`]: exactly the fields an update sets. It is both the update payload and the sparse diff.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct Fem3dModelWindowConfigPatch {
    #[dsl(block)]
    pub camera: Option<crate::Viewport3dOrbit>,
    #[dsl(block)]
    pub gumball: Option<Fem3dGumballConfig>,
}

impl Fem3dModelWindowConfigPatch {
    /// 🩹 The patch that sets every field of `config`.
    pub fn replacing(config: &Fem3dModelWindowConfig) -> Self {
        Self {
            camera: Some(config.camera.clone()),
            gumball: Some(config.gumball.clone()),
        }
    }

    /// 🔎️ This patch reduced to the fields that differ from `base`.
    pub fn against(&self, base: &Fem3dModelWindowConfig) -> Self {
        Self {
            camera: self.camera.clone().filter(|value| *value != base.camera),
            gumball: self.gumball.clone().filter(|value| *value != base.gumball),
        }
    }
}

impl protocol::MutationDiff<Fem3dModelWindowConfig> for Fem3dModelWindowConfigPatch {
    fn apply(&self, base: &Fem3dModelWindowConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Fem3dModelWindowConfig> {
        Ok(Fem3dModelWindowConfig {
            camera: self.camera.clone().unwrap_or_else(|| base.camera.clone()),
            gumball: self.gumball.clone().unwrap_or_else(|| base.gumball.clone()),
        })
    }
    fn absorb(&mut self, other: Self) {
        self.camera = other.camera.or_else(|| self.camera.take());
        self.gumball = other.gumball.or_else(|| self.gumball.take());
    }
}

impl protocol::DiffAlgebra<Fem3dModelWindowConfig> for Fem3dModelWindowConfigPatch {
    fn inverse(&self, base: &Fem3dModelWindowConfig) -> Self {
        Self {
            camera: self.camera.as_ref().map(|_| base.camera.clone()),
            gumball: self.gumball.as_ref().map(|_| base.gumball.clone()),
        }
    }
    fn between(base: &Fem3dModelWindowConfig, other: &Fem3dModelWindowConfig) -> Self {
        Self::replacing(other).against(base)
    }
    fn is_empty(&self) -> bool {
        self.camera.is_none() && self.gumball.is_none()
    }
}
