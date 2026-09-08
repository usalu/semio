
use super::*;

#[semio_framework_async_macros::async_test]
async fn example_fixture_dsl_round_trips() {
    let snapshot = parse_dsl(FLOW_EXAMPLE_TEXT).expect("parse default snapshot");
    store::os_store::test_support::assert_dsl_round_trip(&snapshot);
}

#[semio_framework_async_macros::async_test]
async fn default_snapshot_dsl_round_trips() {
    store::os_store::test_support::assert_dsl_round_trip(&FlowSnapshot::default());
}
