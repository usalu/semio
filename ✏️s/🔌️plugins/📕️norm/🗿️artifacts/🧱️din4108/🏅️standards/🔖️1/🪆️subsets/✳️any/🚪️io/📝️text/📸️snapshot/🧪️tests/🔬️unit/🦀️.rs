use crate::text::snapshot::*;
use semio_framework_test_fixtures::fixture_path;

#[semio_framework_async_macros::async_test]
async fn document_dsl_round_trips() {
    store::os_store::test_support::assert_dsl_round_trip(&Din4108Snapshot::default());
}

#[semio_framework_async_macros::async_test]
async fn bundled_example_fixture_parses_and_round_trips() {
    let text = crate::text::snapshot::encode_din4108_dsl(&Din4108Snapshot::default());
    let document = parse_dsl(&text).expect("parse encoded default");
    store::os_store::test_support::assert_dsl_round_trip(&document);
    // Persist for assets / include_str consumers.
    let root = fixture_path!("../../../🖼️assets");
    let _ = std::fs::create_dir_all(root.join("🎬️demo"));
    let _ = std::fs::create_dir_all(root.join("🎬️failing-thin-insulation"));
    let _ = std::fs::write(root.join("🎬️demo/🗣️.dsl.semio"), &text);
    let _ = std::fs::write(root.join("🎬️demo/🎒️.pack.semio"), crate::binary::snapshot::encode_din4108_pack(&Din4108Snapshot::default()));
    let fail = Din4108Snapshot::failing_thin_insulation();
    let _ = std::fs::write(root.join("🎬️failing-thin-insulation/🗣️.dsl.semio"), crate::text::snapshot::encode_din4108_dsl(&fail));
    let _ = std::fs::write(root.join("🎬️failing-thin-insulation/🎒️.pack.semio"), crate::binary::snapshot::encode_din4108_pack(&fail));
}
