use crate::standards::v1::subsets::any::io::text::snapshot::*;

#[semio_framework_async_macros::async_test]
async fn example_fixture_dsl_round_trips() {
    let snapshot = parse_dsl(runtime_fixture_text("../../../🚪️io/📝️text/📸️snapshot/🧪️tests/🧫️fixtures/🗣️.dsl.semio")).expect("parse default snapshot");
    let expected: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../../🚪️io/📝️text/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json")).expect("neutral snapshot");
    let actual: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&snapshot)).expect("owned JSON matches independent parser");
    assert_eq!(actual, expected);
    store::os_store::test_support::assert_dsl_round_trip(&snapshot);
}

#[semio_framework_async_macros::async_test]
async fn default_snapshot_dsl_round_trips() {
    store::os_store::test_support::assert_dsl_round_trip(&FlowSnapshot::default());
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
