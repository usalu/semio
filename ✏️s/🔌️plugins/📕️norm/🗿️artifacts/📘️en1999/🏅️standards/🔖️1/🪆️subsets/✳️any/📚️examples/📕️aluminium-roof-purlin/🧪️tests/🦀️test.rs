#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️aluminium-roof-purlin.dsl.semio");
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::artifacts::en1999::schema::inferences::En1999Inference;
    use crate::artifacts::en1999::En1999Snapshot;
    use protocol::Inference;
    let snapshot = En1999Snapshot::default();
    assert_eq!(En1999Inference::infer(&snapshot), En1999Inference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::artifacts::en1999::schema::inferences::En1999Inference;
    use crate::artifacts::en1999::En1999Snapshot;
    use protocol::Inference;
    assert_eq!(En1999Inference::infer(&En1999Snapshot::default()), En1999Inference::default());
}
