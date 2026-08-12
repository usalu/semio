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
    let snapshot = <crate::artifacts::lowpoly::LowpolySnapshot as store::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    let inference = crate::artifacts::lowpoly::standards::v1::subsets::any::schema::inferences::LowpolyInference::infer(&snapshot);
    assert_eq!(inference, crate::artifacts::lowpoly::standards::v1::subsets::any::schema::inferences::LowpolyInference::infer(&snapshot));
}

#[test]
fn inference_default_law() {
    use protocol::Inference;
    assert_eq!(
        crate::artifacts::lowpoly::standards::v1::subsets::any::schema::inferences::LowpolyInference::infer(&crate::artifacts::lowpoly::LowpolySnapshot::default()),
        crate::artifacts::lowpoly::standards::v1::subsets::any::schema::inferences::LowpolyInference::default(),
    );
}
//#endregion 🧪️InferenceLaws
