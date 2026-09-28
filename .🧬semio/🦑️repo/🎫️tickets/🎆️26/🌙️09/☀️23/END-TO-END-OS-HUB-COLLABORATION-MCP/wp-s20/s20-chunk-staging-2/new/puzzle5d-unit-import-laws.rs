/// 📥️ LAW: a chunked pick goes through the SDK's own action dispatch: every chunk but the last is staged by
/// the framework (`semio_framework::kernel::ImportStaging`) and edits nothing, and the closing chunk hands
/// `importFixture` the whole file, which lands as ONE undoable edit.
#[semio_framework_async_macros::async_test]
async fn a_chunked_pick_lands_as_one_undoable_edit_through_the_framework_staging() {
    let mut app = app_with_registry();
    dispatch(&mut app, "setActiveExample", Some(&dsl::json!({ "exampleId": "" })), None).expect("empty document");
    assert_eq!(part_count(&app), 0);
    // 📄️ The law needs a document that really chunks — the first shipped one whose export spans more
    // than one host chunk, so it keeps holding whichever example grows past the chunk next.
    let (target, chunks) = [concrete_forest_example_document(), nakagin_example_document()]
        .into_iter()
        .find_map(|document| {
            let chunks = semio_framework::kernel::import_payload_chunks(&export_fixture::puzzle5d_export_json(&document));
            (chunks.len() > 1).then_some((document, chunks))
        })
        .expect("a shipped document whose export spans more than one host chunk");
    for chunk in &chunks {
        let result = dispatch_through_action(&mut app, "importFixture", &semio_framework::kernel::import_chunk_arguments("chunked.json", chunk, None)).expect("import chunk");
        if chunk.chunk + 1 < chunk.chunk_count {
            assert!(result.mutations.is_empty(), "chunk {} of {} staged and must edit nothing", chunk.chunk, chunk.chunk_count);
            assert_eq!(part_count(&app), 0, "chunk {} of {} must leave the document untouched", chunk.chunk, chunk.chunk_count);
        }
    }
    assert_eq!(part_count(&app), target.parts.len(), "the closing chunk landed the whole document");
    dispatch(&mut app, "undo", None, None).expect("undo");
    assert_eq!(part_count(&app), 0, "the whole import is ONE undoable edit");
    close_app(&mut app);
}

/// 📥️ LAW: every refusal answers. A file over the import budget and a payload that is not a puzzle 5d
/// document each publish a named notice and leave the document exactly as it was — never a silent no-op,
/// and never a fault.
#[semio_framework_async_macros::async_test]
async fn every_refused_import_publishes_a_notice_and_changes_nothing() {
    let mut app = app_with_registry();
    let before = projection_of(&app);
    let oversized = "x".repeat(crate::retained_command::PUZZLE_IMPORT_TOTAL_BYTES + 1);
    let cases = [
        // 📦️ One byte above what one export may stream.
        ("capacity", dsl::json!({ "payload": oversized.as_str(), "name": "huge.json" })),
        // 🔤️ A whole file that is not a puzzle 5d document.
        ("payload", dsl::json!({ "payload": "{\"schema\":\"note.v1\",\"body\":\"\"}", "name": "note.json" })),
    ];
    for (case, args) in cases {
        let result = dispatch(&mut app, "importFixture", Some(&args), None).expect("a refused import answers, it never faults");
        assert!(result.mutations.is_empty(), "the {case} refusal emits no mutation");
        assert!(result.requested_effects.iter().any(|effect| matches!(effect, Effect::Notify { .. })), "the {case} refusal is visible: {:?}", result.requested_effects);
        assert_eq!(projection_of(&app), before, "the {case} refusal leaves the document alone");
    }
    close_app(&mut app);
}

/// 🕳️ LAW: a chunk that does not continue its pick is refused BY THE FRAMEWORK with the typed
/// `file-import.gap` fault — the staging never resumes into bytes nobody can account for — and the
/// document stays exactly as it was.
#[semio_framework_async_macros::async_test]
async fn a_gap_in_a_chunked_pick_is_a_typed_framework_refusal() {
    let mut app = app_with_registry();
    let before = projection_of(&app);
    let chunk = semio_framework::kernel::ImportChunk { payload: "\"parts\":[],".into(), chunk: 1, chunk_count: 3 };
    let fault = dispatch_through_action(&mut app, "importFixture", &semio_framework::kernel::import_chunk_arguments("gap.json", &chunk, None)).expect_err("a chunk past the cursor of no open run is refused");
    assert_eq!(fault.code.0, semio_framework::kernel::ImportStagingRefusal::Gap.code(), "the refusal is the framework's typed gap code: {fault:?}");
    assert_eq!(projection_of(&app), before, "a refused chunk leaves the document alone");
    close_app(&mut app);
}
