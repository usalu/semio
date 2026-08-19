#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️glulam-footbridge.dsl.semio");
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::artifacts::en1995::schema::inferences::En1995Inference;
    use crate::artifacts::en1995::En1995Snapshot;
    use protocol::Inference;
    let snapshot = En1995Snapshot::default();
    assert_eq!(En1995Inference::infer(&snapshot), En1995Inference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::artifacts::en1995::schema::inferences::En1995Inference;
    use crate::artifacts::en1995::En1995Snapshot;
    use protocol::Inference;
    assert_eq!(En1995Inference::infer(&En1995Snapshot::default()), En1995Inference::default());
}
