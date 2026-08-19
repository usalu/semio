#[test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    assert!(text.len() > 8);
}

#[test]
async fn inference_determinism_law() {
    use crate::artifacts::writer::schema::inferences::WriterInference;
    use crate::artifacts::writer::WriterSnapshot;
    use protocol::Inference;
    let snapshot = WriterSnapshot::default();
    assert_eq!(WriterInference::infer(&snapshot), WriterInference::infer(&snapshot));
}

#[test]
async fn inference_default_law() {
    use crate::artifacts::writer::schema::inferences::WriterInference;
    use crate::artifacts::writer::WriterSnapshot;
    use protocol::Inference;
    assert_eq!(WriterInference::infer(&WriterSnapshot::default()), WriterInference::default());
}
