//! 🔺️ The sparse delta of the shared Norm Viewport-window configuration: the view a mutation sets.

use crate::schema::NormViewportWindowConfig;

/// 🎯️ The new view state: camera preset, hidden layer ids, colour map and unit system (`None` and empty clear).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ViewState {
    pub camera_preset: Option<String>,
    pub hidden_layers: Vec<String>,
    pub colour_map: Option<String>,
    pub unit_system: Option<String>,
}

impl ViewState {
    pub fn of(config: &NormViewportWindowConfig) -> Self {
        Self { camera_preset: config.camera_preset.clone(), hidden_layers: config.hidden_layers.clone(), colour_map: config.colour_map.clone(), unit_system: config.unit_system.clone() }
    }
}

/// 🔺️ Sparse field delta over [`NormViewportWindowConfig`].
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct NormViewportWindowConfigDiff {
    pub view: Option<ViewState>,
}

impl protocol::MutationDiff<NormViewportWindowConfig> for NormViewportWindowConfigDiff {}

impl protocol::ColdMutationDiff<NormViewportWindowConfig> for NormViewportWindowConfigDiff {
    fn absorb(&mut self, other: Self) {
        if other.view.is_some() {
            self.view = other.view;
        }
    }

    fn apply(&self, unpublished: &mut NormViewportWindowConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        if let Some(view) = &self.view {
            *unpublished = NormViewportWindowConfig { camera_preset: view.camera_preset.clone(), hidden_layers: view.hidden_layers.clone(), colour_map: view.colour_map.clone(), unit_system: view.unit_system.clone() };
        }
        Ok(())
    }
}

impl protocol::DiffAlgebra<NormViewportWindowConfig> for NormViewportWindowConfigDiff {
    fn is_empty(&self) -> bool {
        self.view.is_none()
    }
}

impl protocol::ColdDiffAlgebra<NormViewportWindowConfig> for NormViewportWindowConfigDiff {
    fn inverse(&self, base: &NormViewportWindowConfig) -> Self {
        Self { view: self.view.as_ref().map(|_| ViewState::of(base)) }
    }
}
