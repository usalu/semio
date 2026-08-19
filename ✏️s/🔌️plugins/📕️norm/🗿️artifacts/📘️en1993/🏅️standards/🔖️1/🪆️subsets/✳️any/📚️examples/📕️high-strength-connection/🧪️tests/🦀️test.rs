#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️high-strength-connection.dsl.semio");
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::artifacts::en1993::schema::inferences::En1993Inference;
    use crate::artifacts::en1993::En1993Snapshot;
    use protocol::Inference;
    let snapshot = En1993Snapshot::default();
    assert_eq!(En1993Inference::infer(&snapshot), En1993Inference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::artifacts::en1993::schema::inferences::En1993Inference;
    use crate::artifacts::en1993::En1993Snapshot;
    use protocol::Inference;
    assert_eq!(En1993Inference::infer(&En1993Snapshot::default()), En1993Inference::default());
}
