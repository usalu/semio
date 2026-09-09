#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[test]
fn inference_determinism_law() {
    use crate::standards::v1::subsets::any::schema::inferences::Fem3dInference;
    use protocol::Inference;

    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    let snapshot = crate::standards::v1::subsets::any::schema::snapshot::text::parse_dsl(text).expect("example dsl parses");
    assert_eq!(Fem3dInference::infer(&snapshot), Fem3dInference::infer(&snapshot));
}

#[test]
fn inference_default_law() {
    use crate::standards::v1::subsets::any::schema::inferences::Fem3dInference;
    use crate::Fem3dSnapshot;
    use protocol::Inference;

    assert_eq!(Fem3dInference::infer(&Fem3dSnapshot::default()), Fem3dInference::default());
}
//#endregion 🧪️InferenceLaws
