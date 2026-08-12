#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️high-strength-connection.dsl.semio");
    assert!(text.len() > 8);
}

#[test]
fn inference_determinism_law() {
    use crate::artifacts::en1993::schema::inferences::En1993Inference;
    use crate::artifacts::en1993::En1993Snapshot;
    use protocol::Inference;
    let snapshot = En1993Snapshot::default();
    assert_eq!(En1993Inference::infer(&snapshot), En1993Inference::infer(&snapshot));
}

#[test]
fn inference_default_law() {
    use crate::artifacts::en1993::schema::inferences::En1993Inference;
    use crate::artifacts::en1993::En1993Snapshot;
    use protocol::Inference;
    assert_eq!(En1993Inference::infer(&En1993Snapshot::default()), En1993Inference::default());
}
