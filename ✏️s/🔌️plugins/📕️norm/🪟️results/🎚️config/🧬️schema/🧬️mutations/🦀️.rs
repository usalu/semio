//! 🎚️ The closed semantic mutation vocabulary for Norm Results-window configuration.

use super::change_selected_check_index::ChangeSelectedCheckIndex;
use crate::results_window_config::NormResultsWindowConfig;

#[derive(Clone, Debug, PartialEq, dsl::DslOps, dsl::Mutations, value_derive::ToValue, value_derive::FromValue)]
#[mutations(snapshot = NormResultsWindowConfig, diff = NormResultsWindowConfig, schema = "s.norm.results-window.config")]
pub enum NormResultsWindowConfigMutation {
    ChangeSelectedCheckIndex(ChangeSelectedCheckIndex),
}
