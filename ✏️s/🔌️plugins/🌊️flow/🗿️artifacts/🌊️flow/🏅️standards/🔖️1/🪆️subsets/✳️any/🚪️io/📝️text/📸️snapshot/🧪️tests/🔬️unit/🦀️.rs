use crate::standards::v1::subsets::any::io::text::snapshot::*;

#[semio_framework_async_macros::async_test]
async fn example_fixture_dsl_round_trips() {
    let snapshot = parse_dsl(include_str!("../../🧫️fixtures/🗣️.dsl.semio")).expect("parse default snapshot");
    let expected: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("neutral snapshot");
    let actual: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&snapshot)).expect("owned JSON matches independent parser");
    assert_eq!(actual, expected);
    println!("[DEBUG] Flow literal child DSL matches independent serde_json: child={} target={}", snapshot.content.child_id, snapshot.content.target.artifact_id);
    store::os_store::test_support::assert_dsl_round_trip(&snapshot);
}

#[semio_framework_async_macros::async_test]
async fn default_snapshot_dsl_round_trips() {
    store::os_store::test_support::assert_dsl_round_trip(&FlowSnapshot::default());
}
