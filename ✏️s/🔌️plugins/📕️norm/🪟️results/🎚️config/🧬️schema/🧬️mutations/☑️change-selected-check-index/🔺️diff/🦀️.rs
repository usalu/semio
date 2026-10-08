use super::ChangeSelectedCheckIndex;
use crate::results_window_config::diff::{NormResultsWindowConfigDiff, SelectedCheck};
use crate::results_window_config::NormResultsWindowConfig;

pub fn diff(payload: &ChangeSelectedCheckIndex, base: &NormResultsWindowConfig) -> protocol::MutationOutcome<NormResultsWindowConfigDiff> {
    if base.selected_check_index == payload.index {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Selected check index is already this value.");
    }
    protocol::MutationOutcome::new(NormResultsWindowConfigDiff { selected_check: Some(SelectedCheck { index: payload.index }) })
}
