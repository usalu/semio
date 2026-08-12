#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    assert!(text.len() > 8);
}

#[test]
fn inference_determinism_law() {
    use crate::artifacts::din16798::schema::inferences::Din16798Inference;
    use crate::artifacts::din16798::Din16798Snapshot;
    use protocol::Inference;
    let snapshot = Din16798Snapshot::default();
    assert_eq!(Din16798Inference::infer(&snapshot), Din16798Inference::infer(&snapshot));
}

#[test]
fn inference_default_law() {
    use crate::artifacts::din16798::schema::inferences::Din16798Inference;
    use crate::artifacts::din16798::Din16798Snapshot;
    use protocol::Inference;
    assert_eq!(Din16798Inference::infer(&Din16798Snapshot::default()), Din16798Inference::default());
}
