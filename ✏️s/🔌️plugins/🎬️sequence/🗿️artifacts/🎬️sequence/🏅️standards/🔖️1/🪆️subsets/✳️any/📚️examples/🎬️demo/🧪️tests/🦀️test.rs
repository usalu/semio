#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::artifacts::sequence::standards::v1::subsets::any::schema::inferences::SequenceInference;
    use crate::artifacts::sequence::SequenceSnapshot;
    use protocol::Inference;

    let snapshot = SequenceSnapshot::default();
    assert_eq!(SequenceInference::infer(&snapshot), SequenceInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::artifacts::sequence::standards::v1::subsets::any::schema::inferences::SequenceInference;
    use crate::artifacts::sequence::SequenceSnapshot;
    use protocol::Inference;

    assert_eq!(SequenceInference::infer(&SequenceSnapshot::default()), SequenceInference::default());
}
//#endregion 🧪️InferenceLaws
