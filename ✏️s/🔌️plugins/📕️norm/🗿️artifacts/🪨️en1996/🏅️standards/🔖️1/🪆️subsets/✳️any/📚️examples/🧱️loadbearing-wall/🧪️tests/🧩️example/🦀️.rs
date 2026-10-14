#[test]
fn compliant_example_dsl_asset_parses_and_passes() {
    let text = crate::loadbearing_wall::PRIMARY_TEXT;
    assert!(text.len() > 40, "DSL asset too short");
    let doc = <semio_s_artifact_norm_en1996_model::En1996Snapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).expect("parse loadbearing DSL asset");
    assert_eq!(doc.walls[0].id, "wall-north");
    let report = semio_s_artifact_norm_en1996_inferences::evaluate(&doc);
    assert!(report.complies(), "compliant DSL must Pass: {:?}", report.failing().map(|c| c.id.clone()).collect::<Vec<_>>());
}
