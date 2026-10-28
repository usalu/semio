use super::*;

fn vector_request(kind: LayoutExportKind) -> LayoutExportRequest {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📄️large-sheet/🔣️.json")).unwrap();
    let snapshot: LayoutSnapshot = semio_framework_pack_json::from_json_str(&fixture["document"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    LayoutExportRequest { kind, page_id: Some(snapshot.pages[0].id.clone()), snapshot: Arc::new(snapshot), preflight_json: None, parent_document_id: "large-sheet".into(), canonical_base_revision_hex: "09".repeat(32) }
}

fn vector_operation() -> Operation { Operation::new(semio_framework_job::OperationId(91), semio_framework_job::RevisionId(9), semio_framework_job::Generation(3), 17) }

fn vector_job(request: LayoutExportRequest) -> LayoutExportJob {
    let operation = vector_operation();
    LayoutExportJob::from_admitted(operation, (operation.operation, operation.generation), request, ArtifactOutputChunks::new(MAX_LAYOUT_EXPORT_OUTPUT_BYTES))
}

#[test]
fn large_vector_sheet_validation_and_planning_advance_in_bounded_units() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📄️large-sheet/🔣️.json")).unwrap();
    for kind in [LayoutExportKind::Svg, LayoutExportKind::Pdf] {
        let mut job = vector_job(vector_request(kind));
        let mut validation_units = 0;
        while job.stage == ExportStage::Validate { job.validate_one().unwrap(); validation_units += 1; assert!(validation_units < 10_000); }
        assert!(validation_units >= fixture["frameCount"].as_u64().unwrap() as usize);
        let mut plan_units = 0;
        while job.stage == ExportStage::Plan {
            let before = job.pdf_items.len();
            job.plan_one().unwrap();
            let after = job.pdf_items.len();
            assert!(after - before <= 1, "a frame plan unit must not shape the entire page");
            plan_units += 1; assert!(plan_units < 10_000);
        }
        assert!(plan_units >= fixture["frameCount"].as_u64().unwrap() as usize);
        let xs: Vec<f64> = job.pdf_items.iter().filter_map(|item| if let PdfItem::Rect { x, .. } = item { Some(f64::from(*x)) } else { None }).collect();
        let expected: Vec<f64> = fixture["expectedX"].as_array().unwrap().iter().map(|value| value.as_f64().unwrap()).collect();
        assert_eq!(xs, expected, "layer stacking must survive both vector routes");
        let snapshot_owner = Arc::clone(&job.request.as_ref().unwrap().snapshot);
        let cancelled = drive_job_batch(job, vector_operation(), vector_operation().generation, true, "layout.export.large-sheet.cancel", || ()).0;
        assert_eq!(cancelled, LayoutExportObserved::Cancelled);
        drop(snapshot_owner);
    }
    assert!(run_layout_export_headless_batch(vector_operation(), vector_request(LayoutExportKind::Png)).unwrap_err().contains("raster"));
}

#[test]
fn large_sheet_checkpoints_replay_and_cancellation_interrupts_planning() {
    let operation = vector_operation();
    let request = vector_request(LayoutExportKind::Svg);
    let mut caller = layout_export_fixture_caller(operation, operation.generation);
    let mut job = vector_job(request.clone());
    let mut checkpoint = None;
    for _ in 0..10_000 {
        let mut observed = None;
        caller.turn(LAYOUT_EXPORT_FIXTURE_TURN, |cx| cx.with_original_work_loan(1, u64::MAX, |same| {
            observed = job.step(same)?.as_ref().map(LayoutExportObserved::of_borrow); Ok(())
        })).unwrap();
        if let Some(LayoutExportObserved::Checkpoint { state, .. }) = observed {
            if job.stage == ExportStage::Plan { checkpoint = Some(state); break; }
        }
    }
    let state = checkpoint.expect("a large sheet must yield a checkpoint during frame planning");
    assert!(state.len() <= MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES);
    caller.cancel().cancel_now();
    let mut cancelled = false;
    for _ in 0..64 {
        let observed = caller.turn(LAYOUT_EXPORT_FIXTURE_TURN, |cx| Ok(job.step(cx)?.as_ref().map(LayoutExportObserved::of_borrow))).unwrap();
        if observed == Some(LayoutExportObserved::Cancelled) { cancelled = true; break; }
    }
    assert!(cancelled, "planning cancellation must be observed before another page is shaped");
    job.begin_close();
    for _ in 0..10_000 { if job.terminal_is_empty() { break; } caller.turn(LAYOUT_EXPORT_FIXTURE_TURN, |cx| job.close_step(cx)).unwrap(); }
    assert!(job.terminal_is_empty());
    caller.retire(LAYOUT_EXPORT_FIXTURE_STOCK);
    let restored = LayoutExportJob::restore(operation, request.clone(), &state).unwrap();
    let chunks = restored.output_chunks.as_ref().unwrap().clone();
    let (outcome, resumed) = drive_job_batch(restored, operation, operation.generation, false, "layout.export.large-sheet.restore", || LayoutExportCommit::from_chunks(LayoutExportKind::Svg, "large-sheet", &chunks).unwrap().data);
    assert!(matches!(outcome, LayoutExportObserved::Complete { .. }));
    assert_eq!(resumed, run_layout_export_headless_batch(operation, request).unwrap().data);
}
