#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

/// 🧪️ Every committed example asset must be exactly what this crate's own printer produces for the
/// text the example is minted from. A composed child handle is CONTENT-ADDRESSED
/// (`document_child_handle`), so its `child_id` is also its target's `artifact_id`; a hand-written
/// target (`jack-document`) makes the loaded child unresolvable against the minted member.
#[semio_framework_async_macros::async_test]
async fn every_demo_asset_is_the_printers_own_content_addressed_output() {
    for (label, asset, example) in [
        ("🖼️assets/🎬️demo", crate::document_dsl::JACK_EXAMPLE_TEXT, crate::document_dsl::jack_example_document()),
        ("📚️examples/🎬️demo/🖼️assets/🧪️dag-example", crate::document_dsl::DAG_JACK_EXAMPLE_TEXT, crate::document_dsl::dag_jack_example_document()),
    ] {
        let canonical = crate::document_dsl::print_writer_dsl(&crate::writer_snapshot_with_text(&example.schema, &example.id, &example.language_id, &example.uri, &crate::writer_text(&example)));
        let minted = crate::document_child_handle(&example.id, &crate::writer_text(&example), &example.language_id);
        assert_eq!(minted.child_id, minted.target.artifact_id, "{label}: a content-addressed child handle owns its target artifact id");
        assert_eq!(canonical, asset, "{label}: the committed asset must be this crate's own printed output");
    }
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::schema::inferences::WriterInference;
    use crate::WriterSnapshot;
    use protocol::Inference;
    let snapshot = WriterSnapshot::default();
    assert_eq!(WriterInference::infer(&snapshot), WriterInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::schema::inferences::WriterInference;
    use crate::WriterSnapshot;
    use protocol::Inference;
    assert_eq!(WriterInference::infer(&WriterSnapshot::default()), WriterInference::default());
}
