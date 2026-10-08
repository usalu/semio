use super::ChangeSelectedCheckIndex;
use crate::results_window_config::{NormResultsWindowConfig, NormResultsWindowConfigMutation};

pub fn inverse(_payload: &ChangeSelectedCheckIndex, base: &NormResultsWindowConfig) -> Result<Vec<NormResultsWindowConfigMutation>, semio_framework_value::ValueError> {
    Ok(vec![ChangeSelectedCheckIndex { index: base.selected_check_index }.into()])
}
