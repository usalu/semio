#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[test]
fn inference_determinism_law() {
    use protocol::Inference;
    use crate::artifacts::generation2d::standards::v1::subsets::any::schema::inferences::Generation2dInference;

    let text = include_str!("../🖼️assets/🗣️.dsl.semio");
    let snapshot = crate::artifacts::generation2d::dsl::parse_dsl(text).expect("example dsl parses");
    assert_eq!(Generation2dInference::infer(&snapshot), Generation2dInference::infer(&snapshot));
}

#[test]
fn inference_default_law() {
    use protocol::Inference;
    use crate::artifacts::generation2d::standards::v1::subsets::any::schema::inferences::Generation2dInference;
    use crate::artifacts::generation2d::Generation2dSnapshot;

    assert_eq!(Generation2dInference::infer(&Generation2dSnapshot::default()), Generation2dInference::default());
}
//#endregion 🧪️InferenceLaws
