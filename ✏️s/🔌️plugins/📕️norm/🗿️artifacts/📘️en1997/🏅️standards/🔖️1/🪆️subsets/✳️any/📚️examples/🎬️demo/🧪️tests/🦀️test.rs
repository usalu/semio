#[test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    assert!(text.len() > 8);
}

#[test]
async fn inference_determinism_law() {
    use crate::artifacts::en1997::schema::inferences::En1997Inference;
    use crate::artifacts::en1997::En1997Snapshot;
    use protocol::Inference;
    let snapshot = En1997Snapshot::default();
    assert_eq!(En1997Inference::infer(&snapshot), En1997Inference::infer(&snapshot));
}

#[test]
async fn inference_default_law() {
    use crate::artifacts::en1997::schema::inferences::En1997Inference;
    use crate::artifacts::en1997::En1997Snapshot;
    use protocol::Inference;
    assert_eq!(En1997Inference::infer(&En1997Snapshot::default()), En1997Inference::default());
}
