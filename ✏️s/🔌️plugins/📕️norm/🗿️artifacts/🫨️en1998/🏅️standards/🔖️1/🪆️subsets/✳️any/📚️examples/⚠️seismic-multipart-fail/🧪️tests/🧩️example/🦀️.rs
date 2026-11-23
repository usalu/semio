use semio_s_artifact_norm_en1998_inferences::evaluate;
use semio_s_artifact_norm_en1998_io::text::snapshot::decode_en1998_dsl;

#[test]
fn primary_asset_fails_and_every_failure_has_a_remedy() {
    let snap = decode_en1998_dsl(crate::seismic_multipart_fail::PRIMARY_TEXT).expect("decodes");
    let report = evaluate(&snap);
    let fails: Vec<_> = report.failing().collect();
    assert!(fails.len() >= 2, "fail_count={}", fails.len());
    for check in &fails {
        assert!(!check.remedies.is_empty(), "{} has no remedy", check.id);
    }
}
