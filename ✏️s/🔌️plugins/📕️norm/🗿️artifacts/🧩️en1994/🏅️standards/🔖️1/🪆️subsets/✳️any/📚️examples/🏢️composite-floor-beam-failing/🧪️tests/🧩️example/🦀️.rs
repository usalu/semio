use semio_s_artifact_norm_en1994_inferences::evaluate;
use semio_s_artifact_norm_en1994_io::text::snapshot::decode_en1994_dsl;

#[test]
fn primary_asset_fails_named_checks_with_remedies() {
    let snap = decode_en1994_dsl(crate::composite_floor_beam_failing::PRIMARY_TEXT).expect("decodes");
    let report = evaluate(&snap);
    let fails: Vec<_> = report.failing().collect();
    assert!(fails.len() >= 2, "fail_count={}", fails.len());
    let ids: Vec<_> = fails.iter().map(|c| c.id.as_str()).collect();
    assert!(ids.iter().any(|id| id.contains("mrd") || id.contains("beff") || id.contains("prd") || id.contains("insulation") || id.contains("crack") || id.contains("ltb")), "{ids:?}");
    for check in &fails {
        assert!(!check.remedies.is_empty(), "{} has no remedy", check.id);
    }
}
