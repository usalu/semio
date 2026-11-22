use semio_s_artifact_norm_en1999_inferences::evaluate;
use semio_s_artifact_norm_en1999_io::text::snapshot::decode_en1999_dsl;

#[test]
fn primary_asset_fails_the_claimed_checks_with_remedies() {
    let report = evaluate(&decode_en1999_dsl(crate::noncompliant_multi_fail::PRIMARY_TEXT).expect("noncompliant-multi-fail DSL decodes"));
    let failing: Vec<_> = report.failing().collect();
    for id in ["en1999.6.2.5.m.beam-fail", "en1999.7.2.defl.beam-fail", "en1999.8.5.bolt.bolt-short", "en1999.8.6.weld.weld-thin", "en1999.1-3.fat.fat-hot", "en1999.1-5.buckle.shell-fail"] {
        assert!(failing.iter().any(|c| c.id == id), "{id} must fail; failing: {:?}", failing.iter().map(|c| c.id.clone()).collect::<Vec<_>>());
    }
    for check in &failing {
        assert!(!check.remedies.is_empty(), "{} has no remedy", check.id);
    }
}
