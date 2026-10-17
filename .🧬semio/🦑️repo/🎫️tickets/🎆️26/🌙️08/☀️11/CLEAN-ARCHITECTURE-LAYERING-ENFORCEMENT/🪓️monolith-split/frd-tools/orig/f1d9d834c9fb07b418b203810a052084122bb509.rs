#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = runtime_fixture_text("../../../../../../✳️any/🖼️assets/🎬️demo/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::schema::inferences::NoteInference;
    use crate::NoteSnapshot;
    use protocol::Inference;
    let snapshot = NoteSnapshot::default();
    assert_eq!(NoteInference::infer(&snapshot).expect("valid materialized inference fixture"), NoteInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::schema::inferences::NoteInference;
    use crate::NoteSnapshot;
    use protocol::Inference;
    assert_eq!(NoteInference::infer(&NoteSnapshot::default()).expect("valid materialized inference fixture"), NoteInference::default());
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
