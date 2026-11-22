use semio_s_artifact_norm_en1990_inferences::evaluate;
use semio_s_artifact_norm_en1990_io::text::snapshot::decode_en1990_dsl;

fn decoded() -> semio_s_artifact_norm_en1990_model::En1990Snapshot {
    decode_en1990_dsl(crate::accidental_seismic_failing::PRIMARY_TEXT).expect("accidental-seismic-failing DSL decodes")
}

#[test]
fn primary_asset_decodes_to_the_reference_snapshot() {
    assert_eq!(decoded(), crate::accidental_seismic_failing::reference_snapshot());
}

#[test]
fn decoded_example_fails_the_claimed_checks_with_remedies() {
    let report = evaluate(&decoded());
    let failing: Vec<_> = report.failing().collect();
    for id in ["en1990.6.11b.beam-B1", "en1990.6.12b.beam-B1", "en1990.str.beam-B1.6.10"] {
        assert!(failing.iter().any(|c| c.id == id), "{id} must fail; failing: {:?}", failing.iter().map(|c| c.id.clone()).collect::<Vec<_>>());
    }
    for check in &failing {
        assert!(!check.remedies.is_empty(), "{} has no remedy", check.id);
    }
}
