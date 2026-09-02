#[semio_framework_async_macros::async_test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️high-consequence-office.dsl.semio");
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
fn inference_determinism_law() {
    use crate::artifacts::en1990::schema::inferences::En1990Inference;
    use crate::artifacts::en1990::En1990Snapshot;
    use protocol::Inference;
    let snapshot = En1990Snapshot::default();
    assert_eq!(En1990Inference::infer(&snapshot), En1990Inference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
fn inference_default_law() {
    use crate::artifacts::en1990::schema::inferences::En1990Inference;
    use crate::artifacts::en1990::En1990Snapshot;
    use protocol::Inference;
    assert_eq!(En1990Inference::infer(&En1990Snapshot::default()), En1990Inference::default());
}
