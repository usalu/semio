#[semio_framework_async_macros::async_test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
fn inference_determinism_law() {
    use crate::artifacts::iso16757::schema::inferences::Iso16757Inference;
    use crate::artifacts::iso16757::Iso16757Snapshot;
    use protocol::Inference;
    let snapshot = Iso16757Snapshot::default();
    assert_eq!(Iso16757Inference::infer(&snapshot), Iso16757Inference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
fn inference_default_law() {
    use crate::artifacts::iso16757::schema::inferences::Iso16757Inference;
    use crate::artifacts::iso16757::Iso16757Snapshot;
    use protocol::Inference;
    assert_eq!(Iso16757Inference::infer(&Iso16757Snapshot::default()), Iso16757Inference::default());
}
