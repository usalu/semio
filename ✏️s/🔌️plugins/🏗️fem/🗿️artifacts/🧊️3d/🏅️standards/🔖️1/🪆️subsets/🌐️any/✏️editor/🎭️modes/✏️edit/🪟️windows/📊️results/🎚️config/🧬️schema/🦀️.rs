//! 🧬️ FEM 3D results window-config schema.

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, semio_framework_os_kernel::DslArtifact, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase", deny_unknown_fields, validate = "Self::validate_result_source", validate_controlled = "Self::validate_result_source_controlled")]
#[dsl(layout = "lines")]
#[artifact(id = "fem.3d.resultswindowconfig")]
pub struct Fem3dResultsWindowConfig {
    #[dsl(block)]
    pub camera: semio_framework_os_kernel::Viewport3dOrbit,
    pub result_source_id: Option<String>,
    pub result_mode: semio_s_fem_app_surface::ResultMode,
    pub result_mode_index: u32,
    #[dsl(block)]
    pub animation: semio_s_fem_app_surface::FemResultsAnimation,
}


impl Fem3dResultsWindowConfig {
    /// 🛡️ Result configuration admits only canonical result identities and case-based buckling.
    pub fn validate_result_source(&self) -> Result<(), semio_framework_value::ValueError> {
        validate_source_mode(self.result_source_id.as_deref(), &self.result_mode)
    }
    /// 🎟️ Borrowed source admission consumes one controlled work unit and no scratch allocation.
    pub fn validate_result_source_controlled(&self, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<(), semio_framework_value::ValueError> {
        control.step(1)?;
        self.validate_result_source()
    }
}

fn validate_source_mode(source: Option<&str>, mode: &semio_s_fem_app_surface::ResultMode) -> Result<(), semio_framework_value::ValueError> {
    if let Some(source) = source {
        let kind = semio_s_fem_analyses_model::validate_result_source_identity(source).map_err(|_| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "invalid result source identity"))?;
        if *mode == semio_s_fem_app_surface::ResultMode::Buckling && kind != "loadCase" {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "buckling requires a load case source"));
        }
    }
    Ok(())
}

/// 🩹 Owned-field patch of [`Fem3dResultsWindowConfig`]: exactly the fields an update sets. It is both the update payload and the sparse diff.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct Fem3dResultsWindowConfigPatch {
    #[dsl(block)]
    pub camera: Option<semio_framework_os_kernel::Viewport3dOrbit>,
    #[dsl(block)]
    pub result_source_id: Option<FemResultSourceChange>,
    pub result_mode: Option<semio_s_fem_app_surface::ResultMode>,
    pub result_mode_index: Option<u32>,
    #[dsl(block)]
    pub animation: Option<semio_s_fem_app_surface::FemResultsAnimation>,
}

/// 🔺️ One change of the nullable result source: the inner `None` clears the source.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct FemResultSourceChange {
    pub id: Option<String>,
}

impl Fem3dResultsWindowConfigPatch {
    /// 🩹 The patch that sets every field of `config`.
    pub fn replacing(config: &Fem3dResultsWindowConfig) -> Self {
        Self {
            camera: Some(config.camera.clone()),
            result_source_id: Some(FemResultSourceChange { id: config.result_source_id.clone() }),
            result_mode: Some(config.result_mode.clone()),
            result_mode_index: Some(config.result_mode_index.clone()),
            animation: Some(config.animation.clone()),
        }
    }

    /// 🔎️ This patch reduced to the fields that differ from `base`.
    pub fn against(&self, base: &Fem3dResultsWindowConfig) -> Self {
        Self {
            camera: self.camera.clone().filter(|value| *value != base.camera),
            result_source_id: self.result_source_id.clone().filter(|value| value.id != base.result_source_id),
            result_mode: self.result_mode.clone().filter(|value| *value != base.result_mode),
            result_mode_index: self.result_mode_index.clone().filter(|value| *value != base.result_mode_index),
            animation: self.animation.clone().filter(|value| *value != base.animation),
        }
    }
}

impl protocol::MutationDiff<Fem3dResultsWindowConfig> for Fem3dResultsWindowConfigPatch {

}
impl protocol::ColdMutationDiff<Fem3dResultsWindowConfig> for Fem3dResultsWindowConfigPatch {
    fn absorb(&mut self, other: Self) {
        self.camera = other.camera.or_else(|| self.camera.take());
        self.result_source_id = other.result_source_id.or_else(|| self.result_source_id.take());
        self.result_mode = other.result_mode.or_else(|| self.result_mode.take());
        self.result_mode_index = other.result_mode_index.or_else(|| self.result_mode_index.take());
        self.animation = other.animation.or_else(|| self.animation.take());
    }

    fn apply(&self, base: &mut Fem3dResultsWindowConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let source = self.result_source_id.as_ref().map_or(base.result_source_id.as_deref(), |change| change.id.as_deref());
        let mode = self.result_mode.as_ref().unwrap_or(&base.result_mode);
        validate_source_mode(source, mode).map_err(|_| protocol::MutationApplyError { code: "fem3d.results.source".into(), message: "Invalid result source or mode".into(), target: Vec::new() })?;
        if let Some(value) = &self.camera { base.camera = value.clone(); }
        if let Some(change) = &self.result_source_id { base.result_source_id = change.id.clone(); }
        if let Some(value) = &self.result_mode { base.result_mode = value.clone(); }
        if let Some(value) = &self.result_mode_index { base.result_mode_index = value.clone(); }
        if let Some(value) = &self.animation { base.animation = value.clone(); }
        Ok(())
    }
}

impl protocol::DiffAlgebra<Fem3dResultsWindowConfig> for Fem3dResultsWindowConfigPatch {
    fn is_empty(&self) -> bool {
        self.camera.is_none() && self.result_source_id.is_none() && self.result_mode.is_none() && self.result_mode_index.is_none() && self.animation.is_none()
    }
}
impl protocol::ColdDiffAlgebra<Fem3dResultsWindowConfig> for Fem3dResultsWindowConfigPatch {
    fn inverse(&self, base: &Fem3dResultsWindowConfig) -> Self {
        Self {
            camera: self.camera.as_ref().map(|_| base.camera.clone()),
            result_source_id: self.result_source_id.as_ref().map(|_| FemResultSourceChange { id: base.result_source_id.clone() }),
            result_mode: self.result_mode.as_ref().map(|_| base.result_mode.clone()),
            result_mode_index: self.result_mode_index.as_ref().map(|_| base.result_mode_index.clone()),
            animation: self.animation.as_ref().map(|_| base.animation.clone()),
        }
    }
}
