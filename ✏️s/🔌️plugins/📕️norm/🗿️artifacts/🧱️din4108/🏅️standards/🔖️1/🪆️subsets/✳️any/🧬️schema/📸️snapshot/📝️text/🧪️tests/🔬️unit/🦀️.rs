use super::*;

#[semio_framework_async_macros::async_test]
async fn document_dsl_round_trips() {
    store::os_store::test_support::assert_dsl_round_trip(&Din4108Snapshot::default());
}

#[semio_framework_async_macros::async_test]
async fn bundled_example_fixture_parses_and_round_trips() {
    let text = crate::standards::v1::subsets::any::schema::snapshot::encode_din4108_dsl(&Din4108Snapshot::default());
    let document = parse_dsl(&text).expect("parse encoded default");
    store::os_store::test_support::assert_dsl_round_trip(&document);
    // Persist for assets / include_str consumers.
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets");
    let _ = std::fs::create_dir_all(root.join("🎬️demo"));
    let _ = std::fs::create_dir_all(root.join("🎬️failing-thin-insulation"));
    let _ = std::fs::write(root.join("🎬️demo/🗣️.dsl.semio"), &text);
    let _ = std::fs::write(root.join("🎬️demo/🎒️.pack.semio"), crate::standards::v1::subsets::any::schema::snapshot::encode_din4108_pack(&Din4108Snapshot::default()));
    let fail = Din4108Snapshot::failing_thin_insulation();
    let _ = std::fs::write(root.join("🎬️failing-thin-insulation/🗣️.dsl.semio"), crate::standards::v1::subsets::any::schema::snapshot::encode_din4108_dsl(&fail));
    let _ = std::fs::write(root.join("🎬️failing-thin-insulation/🎒️.pack.semio"), crate::standards::v1::subsets::any::schema::snapshot::encode_din4108_pack(&fail));
}
