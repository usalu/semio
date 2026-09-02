#[semio_framework_async_macros::async_test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️loadbearing-wall.dsl.semio");
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
fn inference_determinism_law() {
    use crate::artifacts::en1996::schema::inferences::En1996Inference;
    use crate::artifacts::en1996::En1996Snapshot;
    use protocol::Inference;
    let snapshot = En1996Snapshot::default();
    assert_eq!(En1996Inference::infer(&snapshot), En1996Inference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
fn inference_default_law() {
    use crate::artifacts::en1996::schema::inferences::En1996Inference;
    use crate::artifacts::en1996::En1996Snapshot;
    use protocol::Inference;
    assert_eq!(En1996Inference::infer(&En1996Snapshot::default()), En1996Inference::default());
}
