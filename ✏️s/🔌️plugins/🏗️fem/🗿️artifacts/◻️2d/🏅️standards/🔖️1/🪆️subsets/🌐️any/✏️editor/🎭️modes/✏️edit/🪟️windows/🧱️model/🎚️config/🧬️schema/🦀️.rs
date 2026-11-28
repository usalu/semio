//! 🧬️ FEM 2D model window-config schema.

use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::gumball::Fem2dGumballConfig;

#[derive(semio_framework_dsl_record_derive::DslRecord, semio_framework_dsl_record_derive::DslReceiving, Clone, Debug, PartialEq, semio_os_kernel::DslArtifact, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(layout = "lines")]
#[artifact(id = "fem.2d.modelwindowconfig")]
pub struct Fem2dModelWindowConfig {
    #[dsl(block)]
    pub camera: semio_os_kernel::Viewport2d,
    #[dsl(block)]
    pub gumball: Fem2dGumballConfig,
}

/// 🩹 Owned-field patch of [`Fem2dModelWindowConfig`]: exactly the fields an update sets. It is both the update payload and the sparse diff.
#[derive(semio_framework_dsl_record_derive::DslRecord, semio_framework_dsl_record_derive::DslReceiving, Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree, semio_framework_value::NativeReceive)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct Fem2dModelWindowConfigPatch {
    #[dsl(block)]
    pub camera: Option<semio_os_kernel::Viewport2d>,
    #[dsl(block)]
    pub gumball: Option<Fem2dGumballConfig>,
}

impl Fem2dModelWindowConfigPatch {
    /// 🩹 The patch that sets every field of `config`.
    pub fn replacing(config: &Fem2dModelWindowConfig) -> Self {
        Self {
            camera: Some(config.camera.clone()),
            gumball: Some(config.gumball),
        }
    }

    /// 🔎️ This patch reduced to the fields that differ from `base`.
    pub fn against(&self, base: &Fem2dModelWindowConfig) -> Self {
        Self {
            camera: self.camera.clone().filter(|value| *value != base.camera),
            gumball: self.gumball.filter(|value| *value != base.gumball),
        }
    }
}

impl protocol::MutationDiff<Fem2dModelWindowConfig> for Fem2dModelWindowConfigPatch {

}
impl protocol::ColdMutationDiff<Fem2dModelWindowConfig> for Fem2dModelWindowConfigPatch {
    fn absorb(&mut self, other: Self) {
        self.camera = other.camera.or_else(|| self.camera.take());
        self.gumball = other.gumball.or_else(|| self.gumball.take());
    }

    fn apply(&self, base: &mut Fem2dModelWindowConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        if let Some(value) = &self.camera { base.camera = value.clone(); }
        if let Some(value) = &self.gumball { base.gumball = *value; }
        Ok(())
    }
}

impl protocol::DiffAlgebra<Fem2dModelWindowConfig> for Fem2dModelWindowConfigPatch {
    fn is_empty(&self) -> bool {
        self.camera.is_none() && self.gumball.is_none()
    }
}
impl protocol::ColdDiffAlgebra<Fem2dModelWindowConfig> for Fem2dModelWindowConfigPatch {
    fn inverse(&self, base: &Fem2dModelWindowConfig) -> Self {
        Self {
            camera: self.camera.as_ref().map(|_| base.camera.clone()),
            gumball: self.gumball.as_ref().map(|_| base.gumball),
        }
    }
}
