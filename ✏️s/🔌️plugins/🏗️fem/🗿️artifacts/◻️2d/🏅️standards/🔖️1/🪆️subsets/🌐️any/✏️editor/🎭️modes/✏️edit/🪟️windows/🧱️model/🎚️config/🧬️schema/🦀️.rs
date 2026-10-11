//! 🧬️ FEM 2D model window-config schema.

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, semio_framework_os_kernel::DslArtifact, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(layout = "lines")]
#[artifact(id = "fem.2d.modelwindowconfig")]
pub struct Fem2dModelWindowConfig {
    #[dsl(block)]
    pub camera: crate::Viewport2d,
}

/// 🩹 Owned-field patch of [`Fem2dModelWindowConfig`]: exactly the fields an update sets. It is both the update payload and the sparse diff.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct Fem2dModelWindowConfigPatch {
    #[dsl(block)]
    pub camera: Option<crate::Viewport2d>,
}

impl Fem2dModelWindowConfigPatch {
    /// 🩹 The patch that sets every field of `config`.
    pub fn replacing(config: &Fem2dModelWindowConfig) -> Self {
        Self {
            camera: Some(config.camera.clone()),
        }
    }

    /// 🔎️ This patch reduced to the fields that differ from `base`.
    pub fn against(&self, base: &Fem2dModelWindowConfig) -> Self {
        Self {
            camera: self.camera.clone().filter(|value| *value != base.camera),
        }
    }
}

impl protocol::MutationDiff<Fem2dModelWindowConfig> for Fem2dModelWindowConfigPatch {
    fn apply(&self, base: &Fem2dModelWindowConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Fem2dModelWindowConfig> {
        Ok(Fem2dModelWindowConfig {
            camera: self.camera.clone().unwrap_or_else(|| base.camera.clone()),
        })
    }
    fn absorb(&mut self, other: Self) {
        self.camera = other.camera.or_else(|| self.camera.take());
    }
}

impl protocol::DiffAlgebra<Fem2dModelWindowConfig> for Fem2dModelWindowConfigPatch {
    fn inverse(&self, base: &Fem2dModelWindowConfig) -> Self {
        Self {
            camera: self.camera.as_ref().map(|_| base.camera.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.camera.is_none()
    }
}
