//! 🧬️ FEM 2D results window-config schema.

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, semio_framework_os_kernel::DslArtifact, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(layout = "lines")]
#[artifact(id = "fem.2d.resultswindowconfig")]
pub struct Fem2dResultsWindowConfig {
    #[dsl(block)]
    pub camera: crate::Viewport2d,
    pub result_source_id: Option<String>,
    pub result_mode: crate::app_surface::ResultMode,
    pub result_mode_index: u32,
    #[dsl(block)]
    pub animation: crate::app_surface::FemResultsAnimation,
}

/// 🩹 Owned-field patch of [`Fem2dResultsWindowConfig`]: exactly the fields an update sets. It is both the update payload and the sparse diff.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct Fem2dResultsWindowConfigPatch {
    #[dsl(block)]
    pub camera: Option<crate::Viewport2d>,
    #[dsl(block)]
    pub result_source_id: Option<FemResultSourceChange>,
    pub result_mode: Option<crate::app_surface::ResultMode>,
    pub result_mode_index: Option<u32>,
    #[dsl(block)]
    pub animation: Option<crate::app_surface::FemResultsAnimation>,
}

/// 🔺️ One change of the nullable result source: the inner `None` clears the source.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct FemResultSourceChange {
    pub id: Option<String>,
}

impl Fem2dResultsWindowConfigPatch {
    /// 🩹 The patch that sets every field of `config`.
    pub fn replacing(config: &Fem2dResultsWindowConfig) -> Self {
        Self {
            camera: Some(config.camera.clone()),
            result_source_id: Some(FemResultSourceChange { id: config.result_source_id.clone() }),
            result_mode: Some(config.result_mode.clone()),
            result_mode_index: Some(config.result_mode_index.clone()),
            animation: Some(config.animation.clone()),
        }
    }

    /// 🔎️ This patch reduced to the fields that differ from `base`.
    pub fn against(&self, base: &Fem2dResultsWindowConfig) -> Self {
        Self {
            camera: self.camera.clone().filter(|value| *value != base.camera),
            result_source_id: self.result_source_id.clone().filter(|value| value.id != base.result_source_id),
            result_mode: self.result_mode.clone().filter(|value| *value != base.result_mode),
            result_mode_index: self.result_mode_index.clone().filter(|value| *value != base.result_mode_index),
            animation: self.animation.clone().filter(|value| *value != base.animation),
        }
    }
}

impl protocol::MutationDiff<Fem2dResultsWindowConfig> for Fem2dResultsWindowConfigPatch {
    fn apply(&self, base: &Fem2dResultsWindowConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Fem2dResultsWindowConfig> {
        Ok(Fem2dResultsWindowConfig {
            camera: self.camera.clone().unwrap_or_else(|| base.camera.clone()),
            result_source_id: self.result_source_id.clone().map_or_else(|| base.result_source_id.clone(), |change| change.id),
            result_mode: self.result_mode.clone().unwrap_or_else(|| base.result_mode.clone()),
            result_mode_index: self.result_mode_index.clone().unwrap_or_else(|| base.result_mode_index.clone()),
            animation: self.animation.clone().unwrap_or_else(|| base.animation.clone()),
        })
    }
    fn absorb(&mut self, other: Self) {
        self.camera = other.camera.or_else(|| self.camera.take());
        self.result_source_id = other.result_source_id.or_else(|| self.result_source_id.take());
        self.result_mode = other.result_mode.or_else(|| self.result_mode.take());
        self.result_mode_index = other.result_mode_index.or_else(|| self.result_mode_index.take());
        self.animation = other.animation.or_else(|| self.animation.take());
    }
}

impl protocol::DiffAlgebra<Fem2dResultsWindowConfig> for Fem2dResultsWindowConfigPatch {
    fn inverse(&self, base: &Fem2dResultsWindowConfig) -> Self {
        Self {
            camera: self.camera.as_ref().map(|_| base.camera.clone()),
            result_source_id: self.result_source_id.as_ref().map(|_| FemResultSourceChange { id: base.result_source_id.clone() }),
            result_mode: self.result_mode.as_ref().map(|_| base.result_mode.clone()),
            result_mode_index: self.result_mode_index.as_ref().map(|_| base.result_mode_index.clone()),
            animation: self.animation.as_ref().map(|_| base.animation.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.camera.is_none() && self.result_source_id.is_none() && self.result_mode.is_none() && self.result_mode_index.is_none() && self.animation.is_none()
    }
}
