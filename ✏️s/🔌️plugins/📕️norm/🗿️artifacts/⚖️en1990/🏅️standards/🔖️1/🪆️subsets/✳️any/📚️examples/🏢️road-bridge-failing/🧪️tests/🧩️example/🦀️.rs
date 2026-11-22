use semio_s_artifact_norm_en1990_inferences::evaluate;
use semio_s_artifact_norm_en1990_io::text::snapshot::decode_en1990_dsl;

fn decoded() -> semio_s_artifact_norm_en1990_model::En1990Snapshot {
    decode_en1990_dsl(crate::road_bridge_failing::PRIMARY_TEXT).expect("road-bridge-failing DSL decodes")
}

#[test]
fn primary_asset_decodes_to_the_reference_snapshot() {
    assert_eq!(decoded(), crate::road_bridge_failing::reference_snapshot());
}

#[test]
fn decoded_example_fails_the_claimed_checks_with_remedies() {
    let report = evaluate(&decoded());
    let failing: Vec<_> = report.failing().collect();
    for id in ["en1990.a14.deflection.deck-D1", "en1990.a2.deflection.deck-D1.sls-D1", "en1990.str.deck-D1.6.10"] {
        assert!(failing.iter().any(|c| c.id == id), "{id} must fail; failing: {:?}", failing.iter().map(|c| c.id.clone()).collect::<Vec<_>>());
    }
    for check in &failing {
        assert!(!check.remedies.is_empty(), "{} has no remedy", check.id);
    }
}
