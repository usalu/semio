use semio_s_artifact_norm_en1994_inferences::evaluate;
use semio_s_artifact_norm_en1994_io::text::snapshot::decode_en1994_dsl;

#[test]
fn decoded_example_has_no_failing_check() {
    let snapshot = decode_en1994_dsl(crate::composite_floor_beam::PRIMARY_TEXT).expect("composite-floor-beam DSL decodes");
    let report = evaluate(&snapshot);
    assert!(report.failing().next().is_none(), "fails: {:#?}", report.failing().map(|c| format!("{} u={:.3} computed={:?} limit={:?}", c.id, c.utilization, c.computed, c.limit)).collect::<Vec<_>>());
    assert!(report.checks.iter().any(|c| c.status == semio_s_plugin_norm_compliance::CheckStatus::Pass), "at least one check must pass");
}
