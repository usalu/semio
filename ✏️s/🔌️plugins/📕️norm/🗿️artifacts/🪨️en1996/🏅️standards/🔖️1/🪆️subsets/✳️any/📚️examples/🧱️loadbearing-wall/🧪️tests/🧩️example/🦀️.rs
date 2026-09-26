#[test]
fn compliant_example_dsl_asset_parses_and_passes() {
    let text = crate::loadbearing_wall::PRIMARY_TEXT;
    assert!(text.len() > 40, "DSL asset too short");
    let doc = <crate::En1996Snapshot as store::ArtifactDsl>::parse_dsl(text).expect("parse loadbearing DSL asset");
    assert_eq!(doc.walls[0].id, "wall-north");
    let report = crate::standards::v1::subsets::any::schema::inferences::evaluate(&doc);
    assert!(report.complies(), "compliant DSL must Pass: {:?}", report.failing().map(|c| c.id.clone()).collect::<Vec<_>>());
}
