use crate::demo::*;
use semio_s_artifact_norm_iso16757_inferences::evaluate;
use pack::snapshot::ArtifactDsl;

#[test]
fn example_id_is_stable() {
    assert!(!ID.is_empty());
    let _ = label();
    assert!(!PRIMARY_TEXT.is_empty());
}

#[test]
fn demo_dsl_decodes_and_complies() {
    let document = <semio_s_artifact_norm_iso16757_model::Iso16757Snapshot as ArtifactDsl>::parse_dsl(PRIMARY_TEXT).expect("decode demo DSL");
    let report = evaluate(&document);
    let failing: Vec<_> = report.failing().map(|c| c.id.as_str()).collect();
    assert!(failing.is_empty(), "demo must have no failing checks: {failing:?}");
    let unverified: Vec<_> = report.unverified().map(|c| c.id.as_str()).collect();
    assert_eq!(unverified, ["iso16757.5.6.1.ifcView"], "only the not-yet-assessable ISO 16757-5 catalogue view may stay unverified");
}
