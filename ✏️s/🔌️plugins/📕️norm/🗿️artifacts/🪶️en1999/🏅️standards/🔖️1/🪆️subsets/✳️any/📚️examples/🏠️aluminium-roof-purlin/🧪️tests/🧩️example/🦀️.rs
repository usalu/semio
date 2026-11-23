use semio_s_artifact_norm_en1999_inferences::evaluate;
use semio_s_artifact_norm_en1999_io::text::snapshot::decode_en1999_dsl;

#[test]
fn primary_asset_complies_with_no_failing_check() {
    let report = evaluate(&decode_en1999_dsl(crate::aluminium_roof_purlin::PRIMARY_TEXT).expect("aluminium-roof-purlin DSL decodes"));
    assert!(!report.checks.is_empty());
    assert!(report.failing().next().is_none(), "fails: {:?}", report.failing().map(|c| c.id.clone()).collect::<Vec<_>>());
}

#[test]
fn primary_asset_is_the_reference_snapshot() {
    assert_eq!(decode_en1999_dsl(crate::aluminium_roof_purlin::PRIMARY_TEXT).expect("aluminium-roof-purlin DSL decodes"), crate::aluminium_roof_purlin::snapshot());
}

#[test]
fn pack_and_json_twins_equal_the_reference_snapshot() {
    use semio_s_artifact_norm_en1999_io::binary::snapshot::{decode_en1999_pack, encode_en1999_pack};
    use semio_s_artifact_norm_en1999_io::text::snapshot::decode_en1999_snapshot_json;
    let snapshot = crate::aluminium_roof_purlin::snapshot();
    let pack = include_bytes!("../../../../🖼️assets/🏠️aluminium-roof-purlin/🎒️.pack.semio");
    assert_eq!(decode_en1999_pack(pack).expect("pack decodes"), snapshot);
    assert_eq!(encode_en1999_pack(&snapshot), pack);
    assert_eq!(decode_en1999_snapshot_json(include_str!("../../../../🖼️assets/🏠️aluminium-roof-purlin/🏠️aluminium-roof-purlin/🔣️.json")).expect("json decodes"), snapshot);
}
