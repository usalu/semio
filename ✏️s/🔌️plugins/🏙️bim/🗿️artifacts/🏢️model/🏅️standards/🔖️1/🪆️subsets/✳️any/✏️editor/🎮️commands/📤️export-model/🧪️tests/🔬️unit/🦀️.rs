use super::*;
use crate::editor::bim::unit_tests::support::{ctx, demo, run};
use protocol::Inference;

fn finished(job: &mut ExportJob, snapshot: &ModelSnapshot) -> (Output, Vec<&'static str>) {
    let mut stages = Vec::new();
    for _ in 0..10_000 {
        match job.advance(snapshot).expect("a step") {
            Advance::Progress(stage) => stages.push(stage),
            Advance::Done(output) => return (output, stages),
        }
    }
    panic!("the job never finished");
}

fn fresh(snapshot: &ModelSnapshot) -> ModelInference {
    ModelInference::infer(snapshot).expect("infers")
}

#[semio_framework_async_macros::async_test]
async fn a_file_stem_is_the_project_name_with_dashes_for_everything_but_letters_and_digits() {
    assert_eq!(stem("Demo House: ground & first"), "Demo-House-ground-first");
    assert_eq!(stem("  Büro  "), "Büro");
    assert_eq!(stem("***"), "model");
}

#[semio_framework_async_macros::async_test]
async fn every_format_is_written_from_the_inference_exactly_like_its_serializer_does() {
    let snapshot = demo();
    let inferred = fresh(&snapshot);
    let ifc_bytes = ifc::export_ifc2x3(&snapshot).expect("the ifc").0;
    let ifc = encode("ifc2x3", &snapshot, &inferred).expect("ifc");
    assert_eq!((ifc.mime_type, ifc.encoding), ("application/x-step", None));
    assert_eq!(ifc.data.as_bytes(), ifc_bytes.as_slice());
    let glb = encode("glb", &snapshot, &inferred).expect("glb");
    assert_eq!(glb.encoding, Some(MEDIA_EXPORT_BASE64_ENCODING));
    assert_eq!(glb.data, semio_framework_io_base64::base64_standard_encode(&gltf::export_glb(&snapshot).expect("the glb").0));
    assert_eq!(encode("svg", &snapshot, &inferred).expect("svg").data, svg::export_svg(&snapshot).expect("the svg"));
    assert_eq!(encode("csv", &snapshot, &inferred).expect("csv").data, csv::report_csv(&snapshot, &inferred));
    assert_eq!(ifc.filename, format!("{}.ifc", stem(&snapshot.project.name)));
}

#[semio_framework_async_macros::async_test]
async fn the_job_reports_progress_in_bounded_steps_and_writes_the_same_file_for_every_format() {
    let snapshot = demo();
    let inferred = fresh(&snapshot);
    for format in FORMATS {
        let instance = semio_framework_plugin::ArtifactInstanceOperationOwnerHandle::detached();
        let mut job = ExportJob::with_steps(format, Some(&instance), 3);
        let (output, stages) = finished(&mut job, &snapshot);
        assert_eq!(output, encode(format, &snapshot, &inferred).expect("the file"), "{format}: the stepped job writes the one-shot file");
        assert_eq!(stages.first(), Some(&"bim-export-infer"));
        assert!(stages.iter().filter(|stage| **stage == "bim-export-infer").count() >= 2, "{format}: three nodes a step need several steps: {stages:?}");
        assert_eq!(stages.last(), Some(&"bim-export-encode"));
        assert_eq!(job.fraction(), 1.0);
    }
}

#[semio_framework_async_macros::async_test]
async fn the_progress_of_a_job_never_decreases() {
    let instance = semio_framework_plugin::ArtifactInstanceOperationOwnerHandle::detached();
    let snapshot = demo();
    let mut job = ExportJob::with_steps("csv", Some(&instance), 2);
    let mut seen = vec![job.fraction()];
    loop {
        match job.advance(&snapshot).expect("a step") {
            Advance::Progress(_) => seen.push(job.fraction()),
            Advance::Done(_) => break,
        }
    }
    assert!(seen.windows(2).all(|pair| pair[0] <= pair[1]), "{seen:?}");
    assert_eq!(seen.first(), Some(&0.0));
    assert_eq!(seen.last(), Some(&1.0));
}

#[semio_framework_async_macros::async_test]
async fn a_cancelled_job_keeps_the_finished_nodes_and_the_next_job_finishes_the_rest() {
    let instance = semio_framework_plugin::ArtifactInstanceOperationOwnerHandle::detached();
    let snapshot = demo();
    let mut cancelled = ExportJob::with_steps("ifc2x3", Some(&instance), 2);
    cancelled.advance(&snapshot).expect("opens");
    cancelled.advance(&snapshot).expect("infers a little");
    assert!(cancelled.fraction() > 0.0 && cancelled.fraction() < 1.0);
    cancelled.cancel(&snapshot);
    let report = inference::report(Some(&instance));
    assert!(report.cancelled && report.computed > 0, "{report:?}");
    let mut next = ExportJob::with_steps("ifc2x3", Some(&instance), 1_000);
    let (output, _) = finished(&mut next, &snapshot);
    assert_eq!(output, encode("ifc2x3", &snapshot, &fresh(&snapshot)).expect("the file"));
    let rest = inference::report(Some(&instance));
    assert!(rest.reused >= report.computed, "the nodes the cancelled job finished were not computed again: {rest:?}");
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_format_is_a_refusal_before_any_work() {
    let instance = semio_framework_plugin::ArtifactInstanceOperationOwnerHandle::detached();
    let snapshot = demo();
    let mut job = ExportJob::new("dwg", Some(&instance));
    assert_eq!(job.advance(&snapshot).expect_err("refused").code.0, "bim.export.format-unknown");
    assert_eq!(encode("dwg", &snapshot, &fresh(&snapshot)).expect_err("refused").code.0, "bim.export.format-unknown");
}

#[semio_framework_async_macros::async_test]
async fn the_command_downloads_the_file_of_the_one_shot_path() {
    let snapshot = demo();
    let mut context = ctx(&[]);
    let emit = run(&snapshot, |doc, cfg| handle(&ExportModel { format: "svg".into(), pressed: Some(true) }, doc, cfg, &mut context)).expect("exports");
    assert!(emit.artifact_mutations.is_empty(), "an export never touches the document");
    let expected = encode("svg", &snapshot, &fresh(&snapshot)).expect("the file");
    match emit.effects.as_slice() {
        [Effect::DownloadMediaExport { filename, mime_type, data, encoding: None }] => assert_eq!((filename.as_str(), mime_type.as_str(), data.as_str()), (expected.filename.as_str(), expected.mime_type, expected.data.as_str())),
        other => panic!("one plain download, got {other:?}"),
    }
    let refused = run(&snapshot, |doc, cfg| handle(&ExportModel { format: "dwg".into(), pressed: None }, doc, cfg, &mut context));
    assert_eq!(refused.err().map(|error| error.code.0.to_string()), Some("bim.export.format-unknown".to_string()));
}
