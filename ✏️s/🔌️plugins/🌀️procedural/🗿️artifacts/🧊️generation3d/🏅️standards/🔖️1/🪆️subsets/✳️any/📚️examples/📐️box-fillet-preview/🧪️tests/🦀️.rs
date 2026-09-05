#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🧪️box-fillet-preview/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[test]
fn inference_determinism_law() {
    use protocol::Inference;
    use crate::artifacts::generation3d::standards::v1::subsets::any::schema::inferences::Generation3dInference;

    let text = include_str!("../🖼️assets/🧪️box-fillet-preview/🗣️.dsl.semio");
    let snapshot = crate::artifacts::generation3d::dsl::parse_dsl(text).expect("example dsl parses");
    assert_eq!(Generation3dInference::infer(&snapshot), Generation3dInference::infer(&snapshot));
}

#[test]
fn inference_default_law() {
    use protocol::Inference;
    use crate::artifacts::generation3d::standards::v1::subsets::any::schema::inferences::Generation3dInference;
    use crate::artifacts::generation3d::Generation3dSnapshot;

    assert_eq!(Generation3dInference::infer(&Generation3dSnapshot::default()), Generation3dInference::default());
}
//#endregion 🧪️InferenceLaws
