#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[test]
fn inference_determinism_law() {
    use crate::artifacts::present::standards::v1::subsets::any::schema::inferences::PresentInference;
    use protocol::Inference;

    let snapshot = crate::artifacts::present::default_present_snapshot();
    assert_eq!(PresentInference::infer(&snapshot), PresentInference::infer(&snapshot));
}

#[test]
fn inference_default_law() {
    use crate::artifacts::present::standards::v1::subsets::any::schema::inferences::PresentInference;
    use crate::artifacts::present::PresentSnapshot;
    use protocol::Inference;

    assert_eq!(PresentInference::infer(&PresentSnapshot::default()), PresentInference::default());
}
//#endregion 🧪️InferenceLaws
