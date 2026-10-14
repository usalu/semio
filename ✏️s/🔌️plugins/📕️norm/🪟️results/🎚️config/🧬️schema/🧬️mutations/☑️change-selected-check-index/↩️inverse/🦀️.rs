use crate::mutations::change_selected_check_index::ChangeSelectedCheckIndex;
use crate::schema::NormResultsWindowConfig;
use crate::mutations::NormResultsWindowConfigMutation;

pub fn inverse(_payload: &ChangeSelectedCheckIndex, base: &NormResultsWindowConfig) -> Result<Vec<NormResultsWindowConfigMutation>, semio_framework_value::ValueError> {
    Ok(vec![ChangeSelectedCheckIndex { index: base.selected_check_index }.into()])
}
