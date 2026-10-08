//! 🔺️ The sparse delta of the shared Norm Results-window configuration: the one field a mutation sets, with its new value.

use crate::results_window_config::NormResultsWindowConfig;

/// 🎯️ The new selected compliance check (`None` clears the selection).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SelectedCheck {
    pub index: Option<u32>,
}

/// 🔺️ Sparse field delta over [`NormResultsWindowConfig`].
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct NormResultsWindowConfigDiff {
    pub selected_check: Option<SelectedCheck>,
}

impl protocol::MutationDiff<NormResultsWindowConfig> for NormResultsWindowConfigDiff {
    fn apply(&self, base: &NormResultsWindowConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<NormResultsWindowConfig> {
        Ok(NormResultsWindowConfig { selected_check_index: self.selected_check.as_ref().map_or(base.selected_check_index, |selected| selected.index) })
    }

    fn absorb(&mut self, other: Self) {
        if other.selected_check.is_some() {
            self.selected_check = other.selected_check;
        }
    }
}

impl protocol::DiffAlgebra<NormResultsWindowConfig> for NormResultsWindowConfigDiff {
    fn inverse(&self, base: &NormResultsWindowConfig) -> Self {
        Self { selected_check: self.selected_check.as_ref().map(|_| SelectedCheck { index: base.selected_check_index }) }
    }

    fn is_empty(&self) -> bool {
        self.selected_check.is_none()
    }
}
