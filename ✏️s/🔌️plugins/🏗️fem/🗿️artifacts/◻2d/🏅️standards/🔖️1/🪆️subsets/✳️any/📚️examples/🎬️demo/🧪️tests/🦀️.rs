#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[test]
fn inference_determinism_law() {
    use crate::artifacts::fem2d::standards::v1::subsets::any::schema::inferences::Fem2dInference;
    use protocol::Inference;

    let text = include_str!("../🖼️assets/🗣️.dsl.semio");
    let snapshot = crate::artifacts::fem2d::dsl::parse_dsl(text).expect("example dsl parses");
    assert_eq!(Fem2dInference::infer(&snapshot), Fem2dInference::infer(&snapshot));
}

#[test]
fn inference_default_law() {
    use crate::artifacts::fem2d::standards::v1::subsets::any::schema::inferences::Fem2dInference;
    use crate::artifacts::fem2d::Fem2dSnapshot;
    use protocol::Inference;

    assert_eq!(Fem2dInference::infer(&Fem2dSnapshot::default()), Fem2dInference::default());
}
//#endregion 🧪️InferenceLaws
