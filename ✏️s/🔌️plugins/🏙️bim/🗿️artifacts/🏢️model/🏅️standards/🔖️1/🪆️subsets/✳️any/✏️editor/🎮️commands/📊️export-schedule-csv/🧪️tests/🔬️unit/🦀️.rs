use super::*;
use crate::editor::bim::unit_tests::support::{ctx, demo, run};
use crate::schedule_kit::preset;
use crate::standards::v1::subsets::any::io::export::csv::schedule_csv;
use crate::ModelInference;
use protocol::Inference;

fn scheduled() -> ModelSnapshot {
    let mut snapshot = demo();
    snapshot.schedules.insert("sch-walls".into(), preset("wall", "Wall list: ground & first").expect("the wall preset"));
    snapshot.schedules.insert("sch-doors".into(), preset("door", "Doors").expect("the door preset"));
    snapshot
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
        match job.advance(snapshot).expect("a step") {
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
    let snapshot = scheduled();
    let expected = schedule_csv(&snapshot, &ModelInference::infer(&snapshot).expect("infers"), "sch-walls").expect("the schedule");
    let mut job = CsvJob::with_steps("sch-walls".into(), Some(7), 3, 2);
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
    let snapshot = scheduled();
    let mut cancelled = CsvJob::with_steps("sch-doors".into(), Some(8), 2, 1);
    for _ in 0..3 {
        cancelled.advance(&snapshot).expect("a step");
    }
    drop(cancelled);
    let mut job = CsvJob::with_steps("sch-doors".into(), Some(8), 64, 64);
    let (text, _) = finished(&mut job, &snapshot);
    assert_eq!(text, schedule_csv(&snapshot, &ModelInference::infer(&snapshot).expect("infers"), "sch-doors").expect("the schedule"));
}

#[semio_framework_async_macros::async_test]
async fn a_job_over_a_schedule_that_left_the_model_stops_with_a_fault() {
    let mut snapshot = scheduled();
    let mut job = CsvJob::new("sch-doors".into(), Some(9));
    job.advance(&snapshot).expect("the first step");
    snapshot.schedules.remove("sch-doors");
    assert_eq!(job.advance(&snapshot).expect_err("gone").code.0, "app.schedule.missing");
}
