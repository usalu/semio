#[test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️composite-bridge-girder.dsl.semio");
    assert!(text.len() > 8);
}

#[test]
async fn inference_determinism_law() {
    use crate::artifacts::en1994::schema::inferences::En1994Inference;
    use crate::artifacts::en1994::En1994Snapshot;
    use protocol::Inference;
    let snapshot = En1994Snapshot::default();
    assert_eq!(En1994Inference::infer(&snapshot), En1994Inference::infer(&snapshot));
}

#[test]
async fn inference_default_law() {
    use crate::artifacts::en1994::schema::inferences::En1994Inference;
    use crate::artifacts::en1994::En1994Snapshot;
    use protocol::Inference;
    assert_eq!(En1994Inference::infer(&En1994Snapshot::default()), En1994Inference::default());
}
