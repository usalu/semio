#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[test]
fn inference_determinism_law() {
    use protocol::Inference;
    use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::inferences::Procedural2dInference;

    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    let snapshot = crate::artifacts::procedural2d::dsl::parse_dsl(text).expect("example dsl parses");
    assert_eq!(Procedural2dInference::infer(&snapshot), Procedural2dInference::infer(&snapshot));
}

#[test]
fn inference_default_law() {
    use protocol::Inference;
    use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::inferences::Procedural2dInference;
    use crate::artifacts::procedural2d::Procedural2dSnapshot;

    assert_eq!(Procedural2dInference::infer(&Procedural2dSnapshot::default()), Procedural2dInference::default());
}
//#endregion 🧪️InferenceLaws
