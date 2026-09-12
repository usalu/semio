use super::*;

#[test]
fn config_set_show_mode_round_trips_and_restores() {
    let base = Generation2dConfig::default();
    let forward = Generation2dConfigMutation::SetShowMode { value: "wire".into() }.diff(&base).into_parts().0;
    assert_eq!(forward.show_mode, "wire");
}

#[test]
fn config_op_text_round_trips_every_variant() {
    let config = Generation2dConfig { ..Generation2dConfig::default() };
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Generation2dConfigMutation::Snapshot { config });
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Generation2dConfigMutation::SetShowMode { value: "generate".into() });
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Generation2dConfigMutation::SetSelectedGeneration { selected_generation_id: None });
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Generation2dConfigMutation::SetSelectedGeneration { selected_generation_id: Some("g1".into()) });
}
