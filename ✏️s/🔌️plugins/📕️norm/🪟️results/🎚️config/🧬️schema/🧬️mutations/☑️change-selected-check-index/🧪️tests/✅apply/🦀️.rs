use super::ChangeSelectedCheckIndex;
use crate::results_window_config::{NormResultsWindowConfig, NormResultsWindowConfigMutation};

#[semio_framework_async_macros::async_test]
async fn selecting_and_clearing_a_check_obeys_the_inverse_sum_law() {
    for (before, index) in [(Some(17), Some(3)), (Some(17), None), (None, Some(0)), (Some(2), Some(2))] {
        let base = NormResultsWindowConfig { selected_check_index: before };
        let mutation: NormResultsWindowConfigMutation = ChangeSelectedCheckIndex { index }.into();
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    }
}
