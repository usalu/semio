/// 🧱️ Wave B59: the payload the browser hands `importFixture` for a REAL example is 145 714 B — the
/// Nakagin Capsule Tower export, 180 objects — and the live `import-distinct` verdict read
/// `paneObjects=180→180` with no notice at all (wave B57 §2.3). Every import law before this one fed a
/// payload of a few hundred bytes, so the size class the product actually imports was never under test.
///
/// 🧩️ The payload reaches the action the way the framework hands it once it has reassembled the host's
/// chunks (`semio_framework::kernel::ImportStaging`): ONE whole `{payload, name}`. The witnesses are the
/// browser's own three: the object census moves, exactly ONE applied history row lands, nothing is refused.
#[semio_framework_async_macros::async_test]
async fn a_one_hundred_forty_five_kilobyte_distinct_fixture_imports_inside_one_settle() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("load the nakagin example");
    let seeded = object_count(&app);
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [220.0, 0.0, 0.0] })), None).await.expect("seed one more object");
    let distinct = crate::editor::puzzle3d::commands::export_fixture::puzzle3d_export_json(&puzzle3d_fixture_from_projection(&projection_of(&app)));
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("return to the example document");
    assert_eq!(object_count(&app), seeded, "the document is back at the example census before the import");
    assert!(distinct.len() > 140_000, "the payload under test must be the product's own size class; observed {} B", distinct.len());
    let (result, settled) = dispatch_reporting(&mut app, "importFixture", Some(&json!({ "payload": distinct.as_str(), "name": "nakagin-capsule-tower-distinct.json" })), None).await;
    let result = result.expect("a product-sized import inside the budget is admitted");
    let notices: Vec<String> = result
        .requested_effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message.clone()),
            _ => None,
        })
        .collect();
    assert!(notices.is_empty(), "a payload inside the declared import budget must not be refused: {notices:?}");
    let rows = history_row_labels(&settled);
    assert_eq!(rows.iter().filter(|row| row.contains("applied=true") && !row.contains("ops=0")).count(), 1, "the whole import records exactly one applied history row: {rows:?}");
    assert_eq!(object_count(&app), seeded + 1, "a product-sized distinct payload must replace the document it was imported over");
}

/// 🧩️ The framework half of an import, end to end for this app: the host's chunk envelope
/// (`semio_framework::kernel::import_chunk_arguments`) goes through the SDK's own action dispatch, whose
/// instance-scoped `ImportStaging` stages every chunk but the last — no command, no edit — and hands
/// `importFixture` the whole file once the run closes: ONE applied history row, the census moved.
#[semio_framework_async_macros::async_test]
async fn a_chunked_pick_reaches_import_fixture_as_one_whole_file_through_the_framework_staging() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("load the nakagin example");
    let seeded = object_count(&app);
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [220.0, 0.0, 0.0] })), None).await.expect("seed one more object");
    let distinct = crate::editor::puzzle3d::commands::export_fixture::puzzle3d_export_json(&puzzle3d_fixture_from_projection(&projection_of(&app)));
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("return to the example document");
    let chunks = semio_framework::kernel::import_payload_chunks(&distinct);
    assert!(chunks.len() > 1, "a product-sized payload spans several host chunks; observed {}", chunks.len());
    let mut applied = 0usize;
    for chunk in &chunks {
        let args = semio_framework::kernel::import_chunk_arguments("nakagin-capsule-tower-distinct.json", chunk, None);
        let (result, settled) = dispatch_action_reporting(&mut app, "importFixture", &args).await;
        result.expect("every chunk of a product-sized pick is admitted");
        let rows = history_row_labels(&settled);
        if chunk.chunk + 1 < chunk.chunk_count {
            assert!(rows.is_empty(), "a staged chunk is no command and no edit; chunk {}: {rows:?}", chunk.chunk);
            assert_eq!(object_count(&app), seeded, "a staged chunk must not move the document; chunk {}", chunk.chunk);
        }
        applied += rows.iter().filter(|row| row.contains("applied=true") && !row.contains("ops=0")).count();
    }
    assert_eq!(applied, 1, "the whole chunked pick records exactly one applied history row");
    assert_eq!(object_count(&app), seeded + 1, "the reassembled file replaced the document it was imported over");
}

/// 🧯️ An import above the budget one export may stream is REFUSED with a notice, never silently dropped
/// and never a fault — the framework hands the whole file, so this budget is the app's one size rule.
#[semio_framework_async_macros::async_test]
async fn an_import_above_the_export_budget_refuses_with_a_notice() {
    use crate::retained_command::PUZZLE_IMPORT_TOTAL_BYTES;
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("load the nakagin example");
    let seeded = object_count(&app);
    let oversized = "x".repeat(PUZZLE_IMPORT_TOTAL_BYTES + 1);
    let (result, settled) = dispatch_reporting(&mut app, "importFixture", Some(&json!({ "payload": oversized.as_str(), "name": "oversized.json" })), None).await;
    let result = result.expect("an over-budget import is an ANSWER, never a fault");
    let notices: Vec<String> = result
        .requested_effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(notices.len(), 1, "an over-budget import publishes exactly one notice: {notices:?}");
    let rows = history_row_labels(&settled);
    assert_eq!(history_rows(&settled), 1, "a refused import logs the command and no edit: {rows:?}");
    assert!(rows.iter().all(|row| row.contains("importFixture") && row.contains("applied=false") && row.contains("ops=0")), "{rows:?}");
    assert_eq!(object_count(&app), seeded, "a refused import leaves the document alone");
}

/// 📤️📥️ LAW: the Nakagin export re-imports as the very document it came from — the whole exported file,
/// read by `puzzle3d_import_value`, rebuilds a fixture whose own export is byte-identical.
#[test]
fn the_nakagin_export_reimports_byte_for_byte() {
    use crate::editor::puzzle3d::commands::import_fixture::puzzle3d_import_value;
    let payload = crate::editor::puzzle3d::commands::export_fixture::puzzle3d_export_json(&NAKAGIN_EXAMPLE_FIXTURE.clone());
    assert!(payload.len() > semio_framework::kernel::IMPORT_CHUNK_BYTES, "the payload under test spans several host chunks: {} B", payload.len());
    let root = puzzle3d_import_value(&json!({ "payload": payload.as_str(), "name": "nakagin.json" })).expect("the whole export is a puzzle 3D document");
    assert_eq!(
        crate::editor::puzzle3d::commands::export_fixture::puzzle3d_export_json(&Puzzle3dFixture::from_value(dsl::json::to_dsl_value(&root)).expect("the imported root IS a puzzle 3D document")),
        payload,
        "the import is byte-identical to the document that was exported"
    );
}
