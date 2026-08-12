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
    let snapshot = <crate::artifacts::cad::CadSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    let inference = crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::CadInference::infer(&snapshot);
    assert_eq!(inference, crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::CadInference::infer(&snapshot));
}

#[test]
fn inference_default_law() {
    use protocol::Inference;
    assert_eq!(
        crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::CadInference::infer(&crate::artifacts::cad::empty_cad_snapshot()),
        crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::CadInference::default(),
    );
}
//#endregion 🧪️InferenceLaws
