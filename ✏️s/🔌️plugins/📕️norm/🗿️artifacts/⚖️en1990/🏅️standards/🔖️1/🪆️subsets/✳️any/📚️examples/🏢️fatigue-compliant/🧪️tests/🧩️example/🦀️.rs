use semio_s_artifact_norm_en1990_inferences::evaluate;
use semio_s_artifact_norm_en1990_io::text::snapshot::decode_en1990_dsl;

fn decoded() -> semio_s_artifact_norm_en1990_model::En1990Snapshot {
    decode_en1990_dsl(crate::fatigue_compliant::PRIMARY_TEXT).expect("fatigue-compliant DSL decodes")
}

#[test]
fn primary_asset_decodes_to_the_reference_snapshot() {
    assert_eq!(decoded(), crate::fatigue_compliant::reference_snapshot());
}

#[test]
fn decoded_example_complies_with_no_failing_check() {
    let report = evaluate(&decoded());
    assert!(!report.checks.is_empty());
    assert!(report.failing().next().is_none(), "fails: {:?}", report.failing().map(|c| c.id.clone()).collect::<Vec<_>>());
    let damage = report.checks.iter().find(|c| c.id == "en1990.fat.damage.beam-B1").expect("fatigue damage check present");
    assert_eq!(damage.status, semio_s_plugin_norm_compliance::CheckStatus::Pass);
}
