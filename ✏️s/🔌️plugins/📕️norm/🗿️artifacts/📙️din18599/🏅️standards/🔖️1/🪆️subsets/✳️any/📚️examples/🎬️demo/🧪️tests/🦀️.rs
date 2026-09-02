#[semio_framework_async_macros::async_test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
fn inference_determinism_law() {
    use crate::artifacts::din18599::schema::inferences::Din18599Inference;
    use crate::artifacts::din18599::Din18599Snapshot;
    use protocol::Inference;
    let snapshot = Din18599Snapshot::default();
    assert_eq!(Din18599Inference::infer(&snapshot), Din18599Inference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
fn inference_default_law() {
    use crate::artifacts::din18599::schema::inferences::Din18599Inference;
    use crate::artifacts::din18599::Din18599Snapshot;
    use protocol::Inference;
    assert_eq!(Din18599Inference::infer(&Din18599Snapshot::default()), Din18599Inference::default());
}
