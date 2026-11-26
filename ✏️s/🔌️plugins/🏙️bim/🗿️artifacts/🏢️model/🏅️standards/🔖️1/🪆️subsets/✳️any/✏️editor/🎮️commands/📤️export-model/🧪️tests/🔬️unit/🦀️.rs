use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::io::export::gltf;
use semio_s_artifact_bim_model_svg::standards::v1::subsets::any::io::export::svg;
use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::ModelInference;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_plugin::kernel::MEDIA_EXPORT_BASE64_ENCODING;
use semio_plugin::Effect;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_kit::editor::bim::kit::fault;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context::BimDispatchCtx;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_bim::editor::bim::{BimCommand, BimModelApp};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::ifc::writer::Schema;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_ifc::standards::v1::subsets::any::io::export::ifc::StagedExport;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_ifc::standards::v1::subsets::any::io::export::ifc;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::Analysis;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
#[allow(unused_imports)]
use semio_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
#[allow(unused_imports)]
use semio_plugin::{ArtifactView, ConfigView, EditorApp, Emit, Fault, NoConfig, NoConfigMutation};
#[allow(unused_imports)]
use value_derive::{FromValue, ToValue};
use semio_s_artifact_bim_model_bim::editor::bim::commands::export_model::{ExportJob, Output, Advance, stem, encode, FORMATS, handle, ExportModel};
use semio_s_artifact_bim_model_bim::editor::bim::fixture::support::{ctx, demo, empty_instance, run};
use protocol::Inference;

fn finished(job: &mut ExportJob, snapshot: &ModelSnapshot) -> (Output, Vec<&'static str>) {
    let mut stages = Vec::new();
    for _ in 0..10_000 {
        match job.advance_cold(snapshot, 3).expect("a step") {
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
async fn the_mounted_work_cancels_and_retires_between_ifc_families() {
    use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller, RevisionId};
    use semio_framework_value::RetainedCloneGrant;
    use semio_plugin::retained_command::ArtifactCommandSnapshotRead;
    use semio_plugin::{AppOperationContext, AppRenderOperationContext, ChildContentView, HistoryView, MountedOwnerPolicyV1};
    use semio_s_artifact_bim_model_bim::editor::bim::commands::export_model::ModelExportWork;
    use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::with_view_inference;

    let snapshot = demo();
    let owner = empty_instance();
    let history = HistoryView::empty();
    let grant = RetainedCloneGrant {
        maximum_items: 1 << 16,
        maximum_copy_bytes: 1 << 24,
        maximum_capacity_bytes: 1 << 24,
        maximum_release_bytes: 1 << 24,
        maximum_depth: 128,
    };
    let operation = AppOperationContext {
        app_instance_id: 1,
        parent_document_id: "bim-model".into(),
        operation_id: 1,
        generation: 1,
        canonical_base_revision: [7; 32],
        retained: grant,
        authoring_seed: String::new(),
    };
    let render = AppRenderOperationContext {
        app_instance_id: operation.app_instance_id,
        base_revision: RevisionId(1),
        canonical_base_revision: operation.canonical_base_revision,
        generation: Generation(operation.generation),
        mounted_policy: MountedOwnerPolicyV1::INTERACTIVE_COMPONENT,
    };
    let doc = ArtifactView::with_render_context(&snapshot, &history, ChildContentView::EMPTY, render, "bim-test", None).await
        .with_committed_render_snapshot(true);
    with_view_inference(Some(&owner), &doc, |_| ());

    let command = BimCommand::ExportModel(ExportModel { format: "ifc4".into(), pressed: None });
    let input = ArtifactCommandInputs::<EditorApp<BimModelApp>> {
        command: &command,
        snapshot: &snapshot,
        snapshot_owner: None,
        snapshot_read: ArtifactCommandSnapshotRead { original: None, operation: &operation, source: None },
        config: &NoConfig::default(),
        history: &history,
        interaction: &protocol::InteractionState::default(),
        hover: &Default::default(),
        context: None,
        operation: &operation,
    };
    let mut work = ModelExportWork::new("exportModel", owner.clone());
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(1), Generation(1), grant, || Some(1), 1000, 1 << 20);
    let mut stages = Vec::new();
    for _ in 0..64 {
        let step = caller.turn(grant, |cx| work.step(&input, cx)).expect("funded export turn");
        let ArtifactCommandWorkStep::Progress { stage, .. } = step else { panic!("the export must first report progress") };
        if stage != "bim-export-infer" {
            stages.push(stage);
        }
        if stages.len() == 2 {
            break;
        }
    }
    assert_eq!(stages, ifc::STAGES[..2].iter().map(|(family, _)| *family).collect::<Vec<_>>());
    assert!(!work.terminal_is_empty());
    caller.cancel().cancel().await;
    let cancelled = caller.turn(grant, |cx| Ok(work.step(&input, cx))).expect("cancellation inspection");
    assert!(cancelled.is_err(), "cancellation must never produce a download");
    work.begin_close();
    for _ in 0..16_384 {
        if work.terminal_is_empty() {
            break;
        }
        caller.turn(grant, |cx| work.close_step(cx)).expect("funded writer retirement");
    }
    assert!(work.terminal_is_empty(), "the partially written IFC graph must retire");
    for _ in 0..16_384 {
        if owner.close_terminal_is_empty().expect("inference state") {
            break;
        }
        caller.turn(grant, |cx| Ok(owner.close_inference_step(cx, grant))).expect("funded inference retirement").expect("the projection closes");
    }
    assert!(owner.close_terminal_is_empty().expect("inference state"));
    caller.retire(grant);
}

#[semio_framework_async_macros::async_test]
async fn rectangular_columns_from_a_cold_mounted_operation_match_both_full_ifc_graphs() {
    use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller};
    use semio_framework_value::RetainedCloneGrant;
    use semio_plugin::retained_command::ArtifactCommandSnapshotRead;
    use semio_plugin::{AppOperationContext, HistoryView};
    use semio_s_artifact_bim_model_bim::editor::bim::commands::export_model::ModelExportWork;
    use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::{Phase, Point2, Profile, Slope, TopConstraint};
    use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{Column, ColumnType, GridLine};

    let source = demo();
    let mut snapshot = ModelSnapshot { sites: source.sites, buildings: source.buildings, storeys: source.storeys, ..ModelSnapshot::default() };
    let mut basement = snapshot.storeys["st-ground"].clone();
    basement.name = "Basement".into();
    basement.level = -1;
    basement.height = 2.6;
    snapshot.grids.cold_insert("grid-a".into(), GridLine {
        building: basement.building.clone(), label: "A".into(), start: Point2 { x: 2.0, y: 0.0 }, end: Point2 { x: 2.0, y: 5.0 },
    });
    snapshot.storeys.cold_insert("st-basement".into(), basement);
    snapshot.column_types.cold_insert("ct".into(), ColumnType {
        name: "Rectangular concrete".into(), profile: Profile::Rectangle { width: 0.35, depth: 0.5 }, material: String::new(),
    });
    for (id, storey, top) in [
        ("col-ground", "st-ground", TopConstraint::StoreyTop { offset: -0.3 }),
        ("col-basement", "st-basement", TopConstraint::Storey { storey: "st-ground".into(), offset: 0.2 }),
    ] {
        snapshot.columns.cold_insert(id.into(), Column {
            name: id.into(), storey: storey.into(), column_type: "ct".into(),
            position: Point2 { x: 2.1, y: 3.2 }, rotation: 0.4, tilt: None, base_offset: 0.1, top, phase: Phase::New,
        });
    }
    let owner = empty_instance();
    let grant = RetainedCloneGrant { maximum_items: 1 << 12, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 128 };
    for (n, (format, cancel_at)) in [("ifc2x3", None), ("ifc4", None), ("ifc4", Some("bim-export-storey-levels")), ("ifc4", Some("frames"))].into_iter().enumerate() {
        let operation_id = 41 + n as u64;
        let operation = AppOperationContext {
            app_instance_id: 1, parent_document_id: "bim-model".into(), operation_id, generation: 1,
            canonical_base_revision: [7; 32], retained: grant, authoring_seed: String::new(),
        };
        let command = BimCommand::ExportModel(ExportModel { format: format.into(), pressed: None });
        let input = ArtifactCommandInputs::<EditorApp<BimModelApp>> {
            command: &command, snapshot: &snapshot, snapshot_owner: None,
            snapshot_read: ArtifactCommandSnapshotRead { original: None, operation: &operation, source: None },
            config: &NoConfig::default(), history: &HistoryView::empty(), interaction: &protocol::InteractionState::default(),
            hover: &Default::default(), context: None, operation: &operation,
        };
        let mut work = ModelExportWork::new("exportModel", owner.clone());
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(operation_id), Generation(1), grant, || Some(1), 1000, 1 << 20);
        let denied = RetainedCloneGrant { maximum_copy_bytes: 1, maximum_capacity_bytes: 0, ..grant };
        let denied = caller.turn(denied, |cx| work.step(&input, cx)).expect("small grant is pending");
        assert!(matches!(denied, ArtifactCommandWorkStep::Pending { stage: "bim-export-column-plan", .. }));
        assert!(work.terminal_is_empty());
        let mut effects = None;
        let mut planned = 0;
        let mut frames = 0;
        for _ in 0..2048 {
            match caller.turn(grant, |cx| work.step(&input, cx)).expect("funded spatial IFC turn") {
                ArtifactCommandWorkStep::Progress { stage, .. } => {
                    planned += usize::from(stage == "bim-export-storey-levels");
                    frames += usize::from(stage == "frames");
                    if cancel_at == Some(stage) && (stage == "frames" || planned == snapshot.storeys.len() + 4) {
                        caller.cancel().cancel().await;
                        let stopped = caller.turn(grant, |cx| Ok(work.step(&input, cx))).expect("inspect cancellation");
                        assert!(stopped.is_err());
                        break;
                    }
                }
                ArtifactCommandWorkStep::Pending { .. } => (),
                ArtifactCommandWorkStep::Complete(output) => { effects = Some(output.effects); break; }
                _ => panic!("unexpected spatial IFC publication"),
            }
        }
        assert!(planned >= snapshot.storeys.len());
        if cancel_at.is_some() {
            assert!(effects.is_none());
        } else {
            assert!(frames >= snapshot.columns.len());
            let effects = effects.expect("spatial IFC publishes a download");
            let [Effect::DownloadMediaExport { data, .. }] = effects.as_slice() else { panic!("one spatial IFC download") };
            let expected = encode(format, &snapshot, &fresh(&snapshot)).expect("the full IFC computation");
            assert_eq!(*data, expected.data);
            assert!(data.contains("IFCBUILDINGSTOREY"));
            assert!(data.contains("IFCGRIDAXIS"));
            assert_eq!(data.matches("=IFCCOLUMN(").count(), snapshot.columns.len());
            assert!(data.contains("IFCEXTRUDEDAREASOLID"));
        }
        work.begin_close();
        for _ in 0..2048 {
            if work.terminal_is_empty() { break; }
            caller.turn(grant, |cx| work.close_step(cx)).expect("spatial IFC closes under funding");
        }
        assert!(work.terminal_is_empty());
        caller.retire(grant);
    }
    snapshot.columns.get_mut("col-ground").expect("the ground column").tilt = Some(Slope { direction: 0.0, angle: 0.1 });
    let command = BimCommand::ExportModel(ExportModel { format: "ifc4".into(), pressed: None });
    let operation = AppOperationContext {
        app_instance_id: 1, parent_document_id: "bim-model".into(), operation_id: 45, generation: 1,
        canonical_base_revision: [8; 32], retained: grant, authoring_seed: String::new(),
    };
    let input = ArtifactCommandInputs::<EditorApp<BimModelApp>> {
        command: &command, snapshot: &snapshot, snapshot_owner: None,
        snapshot_read: ArtifactCommandSnapshotRead::unavailable(&operation),
        config: &NoConfig::default(), history: &HistoryView::empty(), interaction: &protocol::InteractionState::default(),
        hover: &Default::default(), context: None, operation: &operation,
    };
    let mut work = ModelExportWork::new("exportModel", owner.clone());
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(45), Generation(1), grant, || Some(1), 1000, 1 << 20);
    let refusal = (0..128).find_map(|_| {
        match caller.turn(grant, |cx| Ok(work.step(&input, cx))).expect("funded IFC turn") {
            Err(failure) => Some(failure),
            Ok(ArtifactCommandWorkStep::Progress { .. } | ArtifactCommandWorkStep::Pending { .. }) => None,
            Ok(_) => panic!("tilted column should require committed inference"),
        }
    }).expect("a cold tilted column should produce a refusal");
    assert!(refusal.primary_value().is_some_and(|error| error.message.contains("a tilted axis")));
    assert!(work.terminal_is_empty());
    caller.retire(grant);
    assert!(owner.inference_is_empty());
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
    let ifc_bytes = semio_s_artifact_bim_model_ifc::standards::v1::subsets::any::io::export::ifc::export_ifc2x3(&snapshot).expect("the ifc").0;
    let ifc = encode("ifc2x3", &snapshot, &inferred).expect("ifc");
    assert_eq!((ifc.mime_type, ifc.encoding), ("application/x-step", None));
    assert_eq!(ifc.data.as_bytes(), ifc_bytes.as_slice());
    let glb = encode("glb", &snapshot, &inferred).expect("glb");
    assert_eq!(glb.encoding, Some(MEDIA_EXPORT_BASE64_ENCODING));
    assert_eq!(glb.data, semio_framework_io_base64::base64_standard_encode(&gltf::export_glb(&snapshot).expect("the glb").0));
    assert_eq!(encode("svg", &snapshot, &inferred).expect("svg").data, svg::export_svg(&snapshot).expect("the svg"));
    assert_eq!(encode("csv", &snapshot, &inferred).expect("csv").data, semio_s_artifact_bim_model_io::standards::v1::subsets::any::io::export::csv::report_csv(&snapshot, &inferred));
    assert_eq!(ifc.filename, format!("{}.ifc", stem(&snapshot.project.name)));
}

#[semio_framework_async_macros::async_test]
async fn the_job_reports_progress_in_bounded_steps_and_writes_the_same_file_for_every_format() {
    let snapshot = demo();
    let inferred = fresh(&snapshot);
    for format in FORMATS {
        let instance = empty_instance();
        let mut job = ExportJob::new(format.into(), Some(&instance));
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
    let instance = empty_instance();
    let snapshot = demo();
    let mut job = ExportJob::new("csv".into(), Some(&instance));
    let mut seen = vec![job.fraction()];
    loop {
        match job.advance_cold(&snapshot, 3).expect("a step") {
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
    let instance = empty_instance();
    let snapshot = demo();
    let mut cancelled = ExportJob::new("ifc2x3".into(), Some(&instance));
    cancelled.advance_cold(&snapshot, 3).expect("opens");
    cancelled.advance_cold(&snapshot, 3).expect("infers a little");
    assert!(cancelled.fraction() > 0.0 && cancelled.fraction() < 1.0);
    cancelled.cancel_cold(&snapshot);
    let report = semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::report(Some(&instance));
    assert!(report.cancelled && report.computed > 0, "{report:?}");
    let mut next = ExportJob::new("ifc2x3".into(), Some(&instance));
    let (output, _) = finished(&mut next, &snapshot);
    assert_eq!(output, encode("ifc2x3", &snapshot, &fresh(&snapshot)).expect("the file"));
    let rest = semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::report(Some(&instance));
    assert!(rest.reused >= report.computed, "the nodes the cancelled job finished were not computed again: {rest:?}");
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_format_is_a_refusal_before_any_work() {
    let instance = empty_instance();
    let snapshot = demo();
    let mut job = ExportJob::new("dwg".into(), Some(&instance));
    assert_eq!(job.advance_cold(&snapshot, 3).expect_err("refused").code.0, "bim.export.format-unknown");
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
