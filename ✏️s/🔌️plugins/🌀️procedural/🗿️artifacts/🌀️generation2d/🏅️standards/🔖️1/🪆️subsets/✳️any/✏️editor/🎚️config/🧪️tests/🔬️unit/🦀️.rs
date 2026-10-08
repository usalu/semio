use super::*;

#[semio_framework_async_macros::async_test]
async fn config_set_show_mode_round_trips_and_restores() {
    let base = Generation2dConfig::default();
    let mutation = Generation2dConfigMutation::SetShowMode { value: "wire".into() };
    let forward = mutation.diff(&base).into_parts().0;
    assert_eq!(forward, Generation2dConfigDiff { show_mode: Some("wire".into()), ..Default::default() });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn config_selected_generation_set_and_clear_satisfy_the_inverse_sum_law() {
    let selected = Generation2dConfig { selected_generation_id: Some("g1".into()), ..Generation2dConfig::default() };
    for (mutation, base) in [
        (Generation2dConfigMutation::SetSelectedGeneration { selected_generation_id: Some("g2".into()) }, Generation2dConfig::default()),
        (Generation2dConfigMutation::SetSelectedGeneration { selected_generation_id: None }, selected),
    ] {
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    }
}

#[test]
fn config_op_text_round_trips_every_variant() {
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Generation2dConfigMutation::SetShowMode { value: "generate".into() });
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Generation2dConfigMutation::SetSelectedGeneration { selected_generation_id: None });
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Generation2dConfigMutation::SetSelectedGeneration { selected_generation_id: Some("g1".into()) });
}
