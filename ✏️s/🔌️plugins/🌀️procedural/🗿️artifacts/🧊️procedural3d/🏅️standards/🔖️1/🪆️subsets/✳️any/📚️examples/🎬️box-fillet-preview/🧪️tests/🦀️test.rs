#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️box-fillet-preview.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[test]
fn inference_determinism_law() {
    use protocol::Inference;
    use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::inferences::Procedural3dInference;

    let text = include_str!("../🖼️assets/🗣️box-fillet-preview.dsl.semio");
    let snapshot = crate::artifacts::procedural3d::dsl::parse_dsl(text).expect("example dsl parses");
    assert_eq!(Procedural3dInference::infer(&snapshot), Procedural3dInference::infer(&snapshot));
}

#[test]
fn inference_default_law() {
    use protocol::Inference;
    use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::inferences::Procedural3dInference;
    use crate::artifacts::procedural3d::Procedural3dSnapshot;

    assert_eq!(Procedural3dInference::infer(&Procedural3dSnapshot::default()), Procedural3dInference::default());
}
//#endregion 🧪️InferenceLaws
