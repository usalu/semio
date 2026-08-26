#[semio_framework_async_macros::async_test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
fn inference_determinism_law() {
    use crate::artifacts::din4108::schema::inferences::Din4108Inference;
    use crate::artifacts::din4108::Din4108Snapshot;
    use protocol::Inference;
    let snapshot = Din4108Snapshot::default();
    assert_eq!(Din4108Inference::infer(&snapshot), Din4108Inference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
fn inference_default_law() {
    use crate::artifacts::din4108::schema::inferences::Din4108Inference;
    use crate::artifacts::din4108::Din4108Snapshot;
    use protocol::Inference;
    assert_eq!(Din4108Inference::infer(&Din4108Snapshot::default()), Din4108Inference::default());
}
