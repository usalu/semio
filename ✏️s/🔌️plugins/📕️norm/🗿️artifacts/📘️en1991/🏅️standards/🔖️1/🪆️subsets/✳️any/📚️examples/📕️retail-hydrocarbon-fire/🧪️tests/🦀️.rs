#[semio_framework_async_macros::async_test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️retail-hydrocarbon-fire.dsl.semio");
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
fn inference_determinism_law() {
    use crate::artifacts::en1991::schema::inferences::En1991Inference;
    use crate::artifacts::en1991::En1991Snapshot;
    use protocol::Inference;
    let snapshot = En1991Snapshot::default();
    assert_eq!(En1991Inference::infer(&snapshot), En1991Inference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
fn inference_default_law() {
    use crate::artifacts::en1991::schema::inferences::En1991Inference;
    use crate::artifacts::en1991::En1991Snapshot;
    use protocol::Inference;
    assert_eq!(En1991Inference::infer(&En1991Snapshot::default()), En1991Inference::default());
}
