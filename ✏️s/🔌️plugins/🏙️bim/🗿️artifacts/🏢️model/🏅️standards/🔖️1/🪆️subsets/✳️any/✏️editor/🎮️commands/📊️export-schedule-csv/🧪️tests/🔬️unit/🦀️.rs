use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_plugin::{ArtifactView, Effect, Emit, NoConfig, NoConfigMutation};
use semio_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
use semio_s_artifact_bim_model_bim::editor::bim::BimCommand;
use semio_s_artifact_bim_model_bim::editor::bim::commands::export_schedule_csv::{CsvJob, Advance, file_name, resolve, handle, ExportScheduleCsv, ScheduleCsvWork};
use semio_s_artifact_bim_model_bim::editor::bim::fixture::support::{ctx, demo, run};
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::schedule_kit::preset;
use semio_s_artifact_bim_model_io::standards::v1::subsets::any::io::export::csv::schedule_csv;
use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::ModelInference;
use protocol::Inference;

fn scheduled() -> ModelSnapshot {
    let mut snapshot = demo();
    snapshot.schedules.cold_insert("sch-walls".into(), preset("wall", "Wall list: ground & first").expect("the wall preset"));
    snapshot.schedules.cold_insert("sch-doors".into(), preset("door", "Doors").expect("the door preset"));
    snapshot
}

fn mounted() -> semio_plugin::ArtifactInstanceOperationOwnerHandle {
    semio_plugin::ArtifactInstanceOperationOwnerHandle::new(Box::new(semio_plugin::plugin_app_close_prelude::EmptyArtifactInstanceOperationOwner))
}

fn downloaded(emit: &Emit<ModelMutation, NoConfigMutation>) -> (String, String, String) {
    match emit.effects.as_slice() {
        [Effect::DownloadMediaExport { filename, mime_type, data, encoding: None }] => (filename.clone(), mime_type.clone(), data.clone()),
        other => panic!("one plain download, got {other:?}"),
    }
}

fn finished(job: &mut CsvJob, snapshot: &ModelSnapshot) -> (String, Vec<&'static str>) {
    let mut stages = Vec::new();
    for _ in 0..10_000 {
        match job.advance_cold(snapshot, 3, 2).expect("a step") {
            Advance::Progress(stage) => stages.push(stage),
            Advance::Done(text) => return (text, stages),
        }
    }
    panic!("the job never finished");
}

#[semio_framework_async_macros::async_test]
async fn a_file_name_is_the_schedule_name_with_dashes_for_everything_but_letters_and_digits() {
    assert_eq!(file_name("Wall list: ground & first"), "Wall-list-ground-first.csv");
    assert_eq!(file_name("  Türliste  "), "Türliste.csv");
    assert_eq!(file_name("***"), "schedule.csv");
}

#[semio_framework_async_macros::async_test]
async fn the_schedule_is_the_requested_one_else_the_shown_one_else_a_fault() {
    let snapshot = scheduled();
    assert_eq!(resolve("sch-doors", "sch-walls", &snapshot).expect("requested"), "sch-doors");
    assert_eq!(resolve("", "sch-walls", &snapshot).expect("shown"), "sch-walls");
    assert_eq!(resolve("", "", &snapshot).expect_err("none").code.0, "app.schedule.missing");
    assert_eq!(resolve("sch-nope", "sch-walls", &snapshot).expect_err("unknown").code.0, "app.schedule.missing");
}

#[semio_framework_async_macros::async_test]
async fn the_command_downloads_the_csv_the_serializer_writes_for_that_schedule() {
    let snapshot = scheduled();
    let mut context = ctx(&[]);
    context.schedule.schedule = "sch-walls".into();
    let emit = run(&snapshot, |doc, cfg| handle(&ExportScheduleCsv { id: String::new(), pressed: None }, doc, cfg, &mut context)).expect("exports");
    assert!(emit.artifact_mutations.is_empty(), "an export never touches the document");
    let (filename, mime_type, data) = downloaded(&emit);
    assert_eq!((filename.as_str(), mime_type.as_str()), ("Wall-list-ground-first.csv", "text/csv"));
    assert_eq!(data, schedule_csv(&snapshot, &ModelInference::infer(&snapshot).expect("infers"), "sch-walls").expect("the schedule"));
    let missing = run(&snapshot, |doc, cfg| handle(&ExportScheduleCsv { id: "sch-nope".into(), pressed: Some(true) }, doc, cfg, &mut context));
    assert_eq!(missing.err().map(|error| error.code.0.to_string()), Some("app.schedule.missing".to_string()));
}

#[semio_framework_async_macros::async_test]
async fn the_job_reports_progress_in_bounded_steps_and_writes_the_same_file_as_the_one_shot_path() {
    let instance = mounted();
    let snapshot = scheduled();
    let expected = schedule_csv(&snapshot, &ModelInference::infer(&snapshot).expect("infers"), "sch-walls").expect("the schedule");
    let mut job = CsvJob::new("sch-walls".into(), Some(&instance));
    let (text, stages) = finished(&mut job, &snapshot);
    assert_eq!(text, expected, "chunked encoding is the same document");
    assert_eq!(stages.first(), Some(&"bim-schedule-csv-infer"));
    assert!(stages.iter().filter(|stage| **stage == "bim-schedule-csv-infer").count() >= 2, "the inference needs several steps at three nodes a step: {stages:?}");
    assert!(stages.iter().filter(|stage| **stage == "bim-schedule-csv-encode").count() >= 2, "the encoding needs several steps at two records a step: {stages:?}");
    let (written, total) = job.written();
    assert_eq!(written, total);
}

#[semio_framework_async_macros::async_test]
async fn a_cancelled_job_leaves_the_session_correct_and_the_next_job_finishes() {
    let instance = mounted();
    let snapshot = scheduled();
    let mut cancelled = CsvJob::new("sch-doors".into(), Some(&instance));
    for _ in 0..3 {
        cancelled.advance_cold(&snapshot, 3, 2).expect("a step");
    }
    cancelled.cancel_cold(&snapshot);
    let report = semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::report(Some(&instance));
    assert!(report.cancelled && report.computed > 0);
    drop(cancelled);
    let mut job = CsvJob::new("sch-doors".into(), Some(&instance));
    let (text, _) = finished(&mut job, &snapshot);
    assert_eq!(text, schedule_csv(&snapshot, &ModelInference::infer(&snapshot).expect("infers"), "sch-doors").expect("the schedule"));
}

#[semio_framework_async_macros::async_test]
async fn a_job_over_a_schedule_that_left_the_model_stops_with_a_fault() {
    let instance = mounted();
    let mut snapshot = scheduled();
    let mut job = CsvJob::new("sch-doors".into(), Some(&instance));
    job.advance_cold(&snapshot, 3, 2).expect("the first step");
    snapshot.schedules.cold_remove("sch-doors");
    assert_eq!(job.advance_cold(&snapshot, 3, 2).expect_err("gone").code.0, "app.schedule.missing");
}

#[semio_framework_async_macros::async_test]
async fn the_original_schedule_command_drains_the_rendered_csv_and_closes() {
    use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller, RetainedCloneGrant, RevisionId};
    use semio_plugin::{AppOperationContext, AppRenderOperationContext, ChildContentView, HistoryView, MountedOwnerPolicyV1};
    use semio_plugin::retained_command::ArtifactCommandSnapshotRead;
    use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::with_view_inference;

    let mut snapshot = scheduled();
    snapshot.schedules.get_mut("sch-walls").unwrap().columns[0].heading = Some("quoted, \"Tür\"\n".repeat(615));
    let instance = mounted();
    let revision = [7; 32];
    let history = HistoryView::empty();
    let render = AppRenderOperationContext { app_instance_id: 1, base_revision: RevisionId(1), canonical_base_revision: revision, generation: Generation(1), mounted_policy: MountedOwnerPolicyV1::INTERACTIVE_COMPONENT };
    let document = ArtifactView::with_render_context(&snapshot, &history, ChildContentView::EMPTY, render, "bim-test", None).await
        .with_committed_render_snapshot(true);
    let expected = with_view_inference(Some(&instance), &document, |inferred| schedule_csv(&snapshot, inferred, "sch-walls").unwrap());
    let stock = RetainedCloneGrant { maximum_items: 1 << 19, maximum_copy_bytes: 1 << 26, maximum_capacity_bytes: 1 << 26, maximum_release_bytes: 1 << 26, maximum_depth: 256 };
    let grant = RetainedCloneGrant { maximum_items: 256, maximum_copy_bytes: 1 << 17, maximum_capacity_bytes: 1 << 17, maximum_release_bytes: 1 << 17, maximum_depth: 128 };
    let operation = AppOperationContext { app_instance_id: 1, parent_document_id: "bim-test".into(), operation_id: 1, generation: 1, canonical_base_revision: revision, retained: grant, authoring_seed: String::new() };
    let command = BimCommand::ExportScheduleCsv(ExportScheduleCsv { id: "sch-walls".into(), pressed: None });
    let config = NoConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_plugin::app::InteractionHoverState::default();
    let input = ArtifactCommandInputs { command: &command, snapshot: &snapshot, snapshot_owner: None, snapshot_read: ArtifactCommandSnapshotRead::unavailable(&operation), config: &config, history: &history, interaction: &interaction, hover: &hover, context: None, operation: &operation };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(1), Generation(1), stock, || Some(1), 1000, 1 << 22);
    let mut export = ScheduleCsvWork::new("exportScheduleCsv", instance.clone());
    let mut chunks = None;
    assert!(caller.turn(grant, |cx| semio_plugin::app::ArtifactOutputChunks::original_birth_admit(262_144, &mut chunks, cx)).expect("original output birth"));
    assert!(caller.turn(grant, |cx| export.receive_original_output_chunks(&mut chunks, cx)).expect("original output transfer"));
    assert!(chunks.is_none());
    let mut stages = Vec::new();
    let mut output = None;
    for _ in 0..512 {
        let mut step = None;
        caller.turn(grant, |cx| {
            let deadline = cx.original_absolute_deadline_us();
            cx.with_original_work_loan(1, deadline, |cx| {
                step = Some(export.step(&input, cx)?);
                Ok(())
            })
        }).expect("original CSV turn");
        match step.expect("one bounded work step") {
            ArtifactCommandWorkStep::Progress { stage, .. } => stages.push(stage),
            ArtifactCommandWorkStep::Pending { .. } => {},
            ArtifactCommandWorkStep::CompleteDownload { download, .. } => { output = Some(download); break; }
            _ => panic!("schedule completes through one segmented download"),
        }
    }
    let output = output.expect("funded CSV completed");
    assert_eq!(output.filename, "Wall-list-ground-first.csv");
    assert_eq!(output.mime_type, "text/csv");
    assert!(stages.contains(&"bim-schedule-csv-infer"));
    assert!(stages.iter().filter(|stage| **stage == "bim-schedule-csv-encode").count() > 3);
    assert!(output.chunks.chunks_remaining() > 1);
    let mut csv = Vec::new();
    while let Some(page) = output.chunks.take_chunk().expect("read original sealed CSV page") { csv.extend(page); }
    assert_eq!(csv, expected.as_bytes());

    export.begin_close();
    for _ in 0..8192 {
        if export.terminal_is_empty() { break; }
        caller.retirement_turn(grant, |cx| export.close_step(cx)).expect("funded schedule retirement");
    }
    assert!(export.terminal_is_empty());

    let mut cancelled = ScheduleCsvWork::new("exportScheduleCsv", instance.clone());
    assert!(caller.turn(grant, |cx| semio_plugin::app::ArtifactOutputChunks::original_birth_admit(262_144, &mut chunks, cx)).expect("cancelled output birth"));
    assert!(caller.turn(grant, |cx| cancelled.receive_original_output_chunks(&mut chunks, cx)).expect("cancelled output transfer"));
    for _ in 0..100 {
        assert!(matches!(caller.turn(grant, |cx| cancelled.step(&input, cx)).expect("partial CSV work"), ArtifactCommandWorkStep::Progress { .. } | ArtifactCommandWorkStep::Pending { .. }));
    }
    caller.cancel().cancel_now();
    assert!(caller.turn(grant, |cx| cancelled.step(&input, cx)).is_err());
    cancelled.begin_close();
    for _ in 0..8192 {
        if cancelled.terminal_is_empty() { break; }
        caller.retirement_turn(grant, |cx| cancelled.close_step(cx)).expect("cancelled CSV owners retire");
    }
    assert!(cancelled.terminal_is_empty());

    caller.retire(stock);
    let maintenance = RetainedCloneGrant { maximum_copy_bytes: 1 << 16, ..stock };
    let mut close = OriginalJobFixtureCaller::admit(OperationId(2), Generation(1), stock, || Some(1), 1000, 1 << 24);
    for _ in 0..(1 << 18) {
        if instance.close_terminal_is_empty().expect("mounted owner observable") { break; }
        close.retirement_turn(maintenance, |cx| Ok(instance.close_step(cx, maintenance))).expect("funded close turn").expect("mounted projection retires");
    }
    assert!(instance.close_terminal_is_empty().expect("mounted owner observable"), "schedule owner demand {:?}, available {:?}", instance.inference_retirement_demands(maintenance.maximum_copy_bytes), close.remaining_grant());
    close.retire(stock);
}
