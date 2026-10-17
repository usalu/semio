#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = &std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🖼️assets/🏛️bestest-950/🗣️.dsl.semio")).expect("committed example asset");
    assert!(text.len() > 8);
}

/// 🧪️ The committed asset must decode to exactly the case this example claims — the example and
/// the engine's own case catalogue cannot drift apart silently.
#[semio_framework_async_macros::async_test]
async fn asset_carries_the_registered_case_model() {
    let text = &std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🖼️assets/🏛️bestest-950/🗣️.dsl.semio")).expect("committed example asset");
    let snapshot = <crate::snapshot::EnergyModelSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).expect("bestest 950 example parses");
    assert_eq!(snapshot.model, super::model());
}
