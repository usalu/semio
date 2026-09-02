#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[test]
fn inference_determinism_law() {
    use crate::artifacts::presentation::standards::v1::subsets::any::schema::inferences::PresentationInference;
    use protocol::Inference;

    let snapshot = crate::artifacts::presentation::default_presentation_snapshot();
    assert_eq!(PresentationInference::infer(&snapshot), PresentationInference::infer(&snapshot));
}

#[test]
fn inference_default_law() {
    use crate::artifacts::presentation::standards::v1::subsets::any::schema::inferences::PresentationInference;
    use crate::artifacts::presentation::PresentationSnapshot;
    use protocol::Inference;

    assert_eq!(PresentationInference::infer(&PresentationSnapshot::default()), PresentationInference::default());
}
//#endregion 🧪️InferenceLaws
