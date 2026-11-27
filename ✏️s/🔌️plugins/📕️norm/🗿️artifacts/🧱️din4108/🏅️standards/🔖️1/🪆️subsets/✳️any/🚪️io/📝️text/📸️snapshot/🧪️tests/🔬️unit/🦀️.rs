use crate::text::snapshot::*;
use semio_framework_test_fixtures::fixture_path;

#[semio_framework_async_macros::async_test]
async fn document_dsl_round_trips() {
    semio_os_store_test_support::assert_dsl_round_trip(&Din4108Snapshot::default());
}

#[semio_framework_async_macros::async_test]
async fn bundled_examples_preserve_pressure_tests_in_both_carriers() {
    for text in [
        include_str!("../../../../../🖼️assets/🎬️demo/🗣️.dsl.semio"),
        include_str!("../../../../../🖼️assets/🎬️failing-thin-insulation/🗣️.dsl.semio"),
    ] {
        let document = parse_dsl(text).expect("parse bundled example");
        assert_eq!(document.cases.len(), 1);
        semio_os_store_test_support::assert_dsl_round_trip(&document);
        let packed = crate::binary::snapshot::encode_din4108_pack(&document);
        assert_eq!(crate::binary::snapshot::decode_din4108_pack(&packed).expect("decode packed example"), document);
    }
}

#[test]
#[ignore = "refresh checked-in binary examples from the canonical text examples"]
fn regenerate_bundled_binary_examples() {
    let root = fixture_path!("../../../🖼️assets");
    for example in ["🎬️demo", "🎬️failing-thin-insulation"] {
        let path = root.join(example);
        let text = std::fs::read_to_string(path.join("🗣️.dsl.semio")).expect("read bundled example");
        let document = parse_dsl(&text).expect("parse bundled example");
        std::fs::write(path.join("🎒️.pack.semio"), crate::binary::snapshot::encode_din4108_pack(&document)).expect("write packed example");
    }
}
