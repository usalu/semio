#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️seismic-rc-frame.dsl.semio");
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::artifacts::en1998::schema::inferences::En1998Inference;
    use crate::artifacts::en1998::En1998Snapshot;
    use protocol::Inference;
    let snapshot = En1998Snapshot::default();
    assert_eq!(En1998Inference::infer(&snapshot), En1998Inference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::artifacts::en1998::schema::inferences::En1998Inference;
    use crate::artifacts::en1998::En1998Snapshot;
    use protocol::Inference;
    assert_eq!(En1998Inference::infer(&En1998Snapshot::default()), En1998Inference::default());
}
