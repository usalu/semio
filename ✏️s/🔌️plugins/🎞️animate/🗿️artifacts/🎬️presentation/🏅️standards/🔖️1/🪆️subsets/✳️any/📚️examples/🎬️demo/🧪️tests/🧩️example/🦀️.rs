#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[test]
fn inference_determinism_law() {
    use crate::standards::v1::subsets::any::schema::inferences::PresentationInference;
    use protocol::Inference;

    let snapshot = crate::default_presentation_snapshot();
    assert_eq!(PresentationInference::infer(&snapshot).expect("valid materialized inference fixture"), PresentationInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[test]
fn inference_default_law() {
    use crate::standards::v1::subsets::any::schema::inferences::PresentationInference;
    use crate::PresentationSnapshot;
    use protocol::Inference;

    assert_eq!(PresentationInference::infer(&PresentationSnapshot::default()).expect("valid materialized inference fixture"), PresentationInference::default());
}
//#endregion 🧪️InferenceLaws
