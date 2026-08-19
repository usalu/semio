#[test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[test]
async fn inference_determinism_law() {
    use protocol::Inference;
    use crate::artifacts::fem2d::standards::v1::subsets::any::schema::inferences::Fem2dInference;

    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    let snapshot = crate::artifacts::fem2d::dsl::parse_dsl(text).expect("example dsl parses");
    assert_eq!(Fem2dInference::infer(&snapshot), Fem2dInference::infer(&snapshot));
}

#[test]
async fn inference_default_law() {
    use protocol::Inference;
    use crate::artifacts::fem2d::standards::v1::subsets::any::schema::inferences::Fem2dInference;
    use crate::artifacts::fem2d::Fem2dSnapshot;

    assert_eq!(Fem2dInference::infer(&Fem2dSnapshot::default()), Fem2dInference::default());
}
//#endregion 🧪️InferenceLaws
