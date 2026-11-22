use semio_s_artifact_norm_en1997_inferences::evaluate;
use semio_s_artifact_norm_en1997_io::text::snapshot::decode_en1997_dsl;

#[test]
fn primary_asset_fails_the_claimed_checks_with_remedies() {
    let report = evaluate(&decode_en1997_dsl(crate::noncompliant::PRIMARY_TEXT).expect("noncompliant DSL decodes"));
    let failing: Vec<_> = report.failing().collect();
    for id in ["en1997.6.5.bearing.footing-F1.lc-bsP", "en1997.7.6.2.compression.pile-P1", "en1997.9.overturning.wall-W1", "en1997.11.bishop.slope-S1", "en1997.10.upl.upl-U1"] {
        assert!(failing.iter().any(|c| c.id == id), "{id} must fail; failing: {:?}", failing.iter().map(|c| c.id.clone()).collect::<Vec<_>>());
    }
    for check in &failing {
        assert!(!check.remedies.is_empty(), "{} has no remedy", check.id);
    }
}
