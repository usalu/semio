#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️liquid-retaining-fem-anchor.dsl.semio");
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::artifacts::en1992::schema::inferences::En1992Inference;
    use crate::artifacts::en1992::En1992Snapshot;
    use protocol::Inference;
    let snapshot = En1992Snapshot::default();
    assert_eq!(En1992Inference::infer(&snapshot), En1992Inference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::artifacts::en1992::schema::inferences::En1992Inference;
    use crate::artifacts::en1992::En1992Snapshot;
    use protocol::Inference;
    assert_eq!(En1992Inference::infer(&En1992Snapshot::default()), En1992Inference::default());
}
