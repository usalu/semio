#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[test]
fn inference_determinism_law() {
    use protocol::Inference;
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    let snapshot = <crate::artifacts::gisterrain::GisTerrainSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    let inference = crate::artifacts::gisterrain::standards::v1::subsets::any::schema::inferences::GisTerrainInference::infer(&snapshot);
    assert_eq!(inference, crate::artifacts::gisterrain::standards::v1::subsets::any::schema::inferences::GisTerrainInference::infer(&snapshot));
}

#[test]
fn inference_default_law() {
    use protocol::Inference;
    assert_eq!(
        crate::artifacts::gisterrain::standards::v1::subsets::any::schema::inferences::GisTerrainInference::infer(&crate::artifacts::gisterrain::GisTerrainSnapshot::default()),
        crate::artifacts::gisterrain::standards::v1::subsets::any::schema::inferences::GisTerrainInference::default(),
    );
}
//#endregion 🧪️InferenceLaws
