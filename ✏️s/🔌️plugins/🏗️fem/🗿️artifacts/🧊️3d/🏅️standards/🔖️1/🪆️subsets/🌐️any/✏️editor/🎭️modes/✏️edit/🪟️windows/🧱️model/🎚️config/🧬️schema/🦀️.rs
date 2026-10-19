//! 🧬️ FEM 3D model window-config schema.

use semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::gumball::Fem3dGumballConfig;

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, semio_framework_os_kernel::DslArtifact, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(layout = "lines")]
#[artifact(id = "fem.3d.modelwindowconfig")]
pub struct Fem3dModelWindowConfig {
    #[dsl(block)]
    pub camera: semio_framework_os_kernel::Viewport3dOrbit,
    #[dsl(block)]
    pub gumball: Fem3dGumballConfig,
}

/// 🩹 Owned-field patch of [`Fem3dModelWindowConfig`]: exactly the fields an update sets. It is both the update payload and the sparse diff.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct Fem3dModelWindowConfigPatch {
    #[dsl(block)]
    pub camera: Option<semio_framework_os_kernel::Viewport3dOrbit>,
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

}
impl protocol::ColdMutationDiff<Fem3dModelWindowConfig> for Fem3dModelWindowConfigPatch {
    fn absorb(&mut self, other: Self) {
        self.camera = other.camera.or_else(|| self.camera.take());
        self.gumball = other.gumball.or_else(|| self.gumball.take());
    }

    fn apply(&self, base: &mut Fem3dModelWindowConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        if let Some(value) = &self.camera { base.camera = value.clone(); }
        if let Some(value) = &self.gumball { base.gumball = value.clone(); }
        Ok(())
    }
}

impl protocol::DiffAlgebra<Fem3dModelWindowConfig> for Fem3dModelWindowConfigPatch {
    fn is_empty(&self) -> bool {
        self.camera.is_none() && self.gumball.is_none()
    }
}
impl protocol::ColdDiffAlgebra<Fem3dModelWindowConfig> for Fem3dModelWindowConfigPatch {
    fn inverse(&self, base: &Fem3dModelWindowConfig) -> Self {
        Self {
            camera: self.camera.as_ref().map(|_| base.camera.clone()),
            gumball: self.gumball.as_ref().map(|_| base.gumball.clone()),
        }
    }
}
