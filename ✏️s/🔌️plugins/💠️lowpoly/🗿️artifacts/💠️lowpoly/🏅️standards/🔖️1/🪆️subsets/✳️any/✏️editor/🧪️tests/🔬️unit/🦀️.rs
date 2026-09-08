
use super::*;
use crate::editor::lowpoly::testkit::{LowpolyApp, app, app_with_registry};
use semio_framework_plugin::{EditorApp, PluginApp, testkit};

fn retained_operation() -> AppOperationContext {
    AppOperationContext { app_instance_id: 7, parent_document_id: "lowpoly-retained-test".into(), operation_id: 11, generation: 13, canonical_base_revision: [17; 32] }
}

fn retained_context(transient: LowpolyTransient, transient_generation: u64) -> std::sync::Arc<ArtifactOwnedToolJobContext<EditorApp<LowpolyPlayApp>>> {
    std::sync::Arc::new(ArtifactOwnedToolJobContext::new(7, None, [17; 32], 0, transient_generation, std::sync::Arc::new(semio_framework_plugin::ChildContentView::EMPTY), std::sync::Arc::new(NoDraft::default()), std::sync::Arc::new(transient)))
}

#[test]
fn retained_route_partition_and_publication_are_exact() {
    use semio_framework::{ToolCancellationPolicy, ToolExecutionShape};

    let all = every_command();
    let mut partition = LOWPOLY_MIGRATED_TOOL_IDS.to_vec();
    partition.sort_unstable();
    partition.dedup();
    assert_eq!(partition.len(), 47);
    assert_eq!(all.len(), partition.len());
    assert!(all.iter().all(|command| partition.binary_search(&command.command_id()).is_ok()));
    assert!(LOWPOLY_MIGRATED_TOOL_IDS.iter().all(|tool_id| lowpoly_command_disposition(tool_id).is_some()));
    assert_eq!(<LowpolyPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), 47);
    assert_eq!(<LowpolyCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.len(), 47);
    assert_eq!(lowpoly_contract().shape, ToolExecutionShape::Resumable);
    assert_eq!(lowpoly_contract().cancellation, ToolCancellationPolicy::PerOperation);
    assert_eq!((lowpoly_contract().checkpoint_every_steps, lowpoly_contract().progress_every_steps), (1, 1));
}

#[semio_framework_async_macros::async_test]
async fn retained_progress_replay_freshness_and_close_are_exact() {
    let command = LowpolyCommand::ToggleShowEdges(toggle_show_edges::ToggleShowEdges {});
    let snapshot = crate::schema::default_snapshot();
    let config = LowpolyConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let history = HistoryView::empty();
    let operation = retained_operation();
    let context = retained_context(LowpolyTransient::default(), 19);
    let context_identity = context.identity_digest();
    let mut uninterrupted = LowpolyRetainedCommandWork::new("toggleShowEdges", LowpolyCommandDisposition::Config, operation.operation_id, operation.generation, operation.canonical_base_revision, context_identity);
    assert!(matches!(
        uninterrupted
            .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs {
                command: &command,
                snapshot: &snapshot,
                config: &config,
                history: &history,
                interaction: &interaction,
                hover: &hover,
                context: Some(&context),
                operation: &operation
            })
            .expect("progress"),
        ArtifactCommandWorkStep::Progress { .. }
    ));
    let mut checkpoint = [0_u8; 88];
    uninterrupted.checkpoint(&mut checkpoint).expect("checkpoint");
    let mut wrong_base = LowpolyRetainedCommandWork::new("toggleShowEdges", LowpolyCommandDisposition::Config, operation.operation_id, operation.generation, [18; 32], context_identity);
    assert!(wrong_base.restore(&checkpoint).is_err());
    let mut replayed = LowpolyRetainedCommandWork::new("toggleShowEdges", LowpolyCommandDisposition::Config, operation.operation_id, operation.generation, operation.canonical_base_revision, context_identity);
    replayed.restore(&checkpoint).expect("work restore");
    assert!(matches!(
        replayed
            .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs {
                command: &command,
                snapshot: &snapshot,
                config: &config,
                history: &history,
                interaction: &interaction,
                hover: &hover,
                context: Some(&context),
                operation: &operation
            })
            .expect("replay"),
        ArtifactCommandWorkStep::Replay { .. }
    ));
    assert!(matches!(
        replayed
            .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs {
                command: &command,
                snapshot: &snapshot,
                config: &config,
                history: &history,
                interaction: &interaction,
                hover: &hover,
                context: Some(&context),
                operation: &operation
            })
            .expect("complete"),
        ArtifactCommandWorkStep::Complete(_)
    ));
    let drifted = AppOperationContext { generation: operation.generation + 1, ..operation.clone() };
    let mut rejected = LowpolyRetainedCommandWork::new("toggleShowEdges", LowpolyCommandDisposition::Config, operation.operation_id, operation.generation, operation.canonical_base_revision, context_identity);
    assert!(
        rejected
            .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs { command: &command, snapshot: &snapshot, config: &config, history: &history, interaction: &interaction, hover: &hover, context: Some(&context), operation: &drifted })
            .is_err()
    );
    let drifted_context = retained_context(LowpolyTransient::default(), 20);
    assert!(
        rejected
            .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs {
                command: &command,
                snapshot: &snapshot,
                config: &config,
                history: &history,
                interaction: &interaction,
                hover: &hover,
                context: Some(&drifted_context),
                operation: &operation
            })
            .is_err()
    );
    assert!(matches!(
        rejected
            .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs {
                command: &command,
                snapshot: &snapshot,
                config: &config,
                history: &history,
                interaction: &interaction,
                hover: &hover,
                context: Some(&context),
                operation: &operation
            })
            .expect("exact retry"),
        ArtifactCommandWorkStep::Progress { .. }
    ));
    assert_eq!(replayed.close_step(0, 0), InteractiveJobCloseStep::Blocked);
    replayed.begin_close();
    assert_eq!(replayed.close_step(1, 1), InteractiveJobCloseStep::Complete);
    assert!(replayed.terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn retained_migrated_turns_stay_below_eight_milliseconds() {
    let snapshot = crate::schema::default_snapshot();
    let config = LowpolyConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let history = HistoryView::empty();
    let operation = retained_operation();
    let context = retained_context(LowpolyTransient::default(), 29);
    for command in every_command().into_iter().filter(|command| LOWPOLY_MIGRATED_TOOL_IDS.contains(&command.command_id())) {
        let tool_id = command.command_id();
        let disposition = lowpoly_command_disposition(tool_id).expect("migrated disposition");
        let mut work = LowpolyRetainedCommandWork::new(tool_id, disposition, operation.operation_id, operation.generation, operation.canonical_base_revision, context.identity_digest());
        loop {
            let started = std::time::Instant::now();
            let step = work
                .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs {
                    command: &command,
                    snapshot: &snapshot,
                    config: &config,
                    history: &history,
                    interaction: &interaction,
                    hover: &hover,
                    context: Some(&context),
                    operation: &operation,
                })
                .expect("migrated turn");
            assert!(started.elapsed() < std::time::Duration::from_millis(8), "{tool_id} turn exceeded 8 ms");
            if matches!(step, ArtifactCommandWorkStep::Complete(_) | ArtifactCommandWorkStep::CompleteWithEphemeral { .. }) {
                break;
            }
        }
        work.begin_close();
        assert_eq!(work.close_step(1, LOWPOLY_ARTIFACT_STORE_MAXIMUM_BYTES), InteractiveJobCloseStep::Complete);
    }
}

//#region 🔖️CommandSurface
/// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every row's
/// wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
#[semio_framework_async_macros::async_test]
async fn command_ids_are_unique() {
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids.len(), 47, "every LowpolyCommand row must be covered by every_command()");
}

/// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_through_text_and_binary() {
    for command in every_command() {
        semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword.
#[semio_framework_async_macros::async_test]
async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
    for command in every_command() {
        let printed = protocol::OpText::print_op(&command);
        let first_token = printed.split(' ').next().unwrap_or_default();
        assert!(!first_token.is_empty(), "printed op line must start with a wire keyword: {printed:?}");
    }
}

/// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
pub(super) fn every_command() -> Vec<LowpolyCommand> {
    vec![
        LowpolyCommand::AddPrimitive(add_primitive::AddPrimitive { kind: Some("box".into()) }),
        LowpolyCommand::PatchObject(patch_object::PatchObject { object_id: "obj-1".into(), field: "name".into(), value_json: Some("\"Renamed\"".into()) }),
        LowpolyCommand::Extrude(extrude::Extrude { extrude_distance: Some(0.25) }),
        LowpolyCommand::Inset(inset::Inset { inset_amount: Some(0.1) }),
        LowpolyCommand::Bevel(bevel::Bevel { bevel_amount: Some(0.05), bevel_segments: Some(1) }),
        LowpolyCommand::LoopCut(loop_cut::LoopCut { loop_cuts: Some(1) }),
        LowpolyCommand::Subdivide(subdivide::Subdivide {}),
        LowpolyCommand::Triangulate(triangulate::Triangulate {}),
        LowpolyCommand::Mirror(mirror::Mirror { axis: Some("x".into()) }),
        LowpolyCommand::Decimate(decimate::Decimate { decimate_ratio: Some(0.5) }),
        LowpolyCommand::FlipFaces(flip_faces::FlipFaces { face_ids: vec![0] }),
        LowpolyCommand::Merge(merge::Merge {}),
        LowpolyCommand::Dissolve(dissolve::Dissolve {}),
        LowpolyCommand::Snap(snap::Snap {}),
        LowpolyCommand::ToggleSmooth(toggle_smooth::ToggleSmooth {}),
        LowpolyCommand::UnwrapActive(unwrap_active::UnwrapActive {}),
        LowpolyCommand::MarkUvSeam(mark_uv_seam::MarkUvSeam { seam: Some(true), edge_ids: Some(vec![0]) }),
        LowpolyCommand::ClearSeam(clear_seam::ClearSeam {}),
        LowpolyCommand::TranslateSelection(translate_selection::TranslateSelection { mode: Some("mesh".into()), ids: Some(vec![]), dx: 1.0, dy: 0.0, dz: 0.0 }),
        LowpolyCommand::RotateSelection(rotate_selection::RotateSelection { mode: Some("mesh".into()), ids: Some(vec![]), ax: 0.0, ay: 1.0, az: 0.0, angle: 45.0 }),
        LowpolyCommand::ScaleSelection(scale_selection::ScaleSelection { mode: Some("mesh".into()), ids: Some(vec![]), sx: 1.0, sy: 1.0, sz: 1.0 }),
        LowpolyCommand::AddPaintLayer(add_paint_layer::AddPaintLayer { object_id: None, name: Some("Detail".into()) }),
        LowpolyCommand::PaintStrokeEnd(paint_stroke_end::PaintStrokeEnd {}),
        LowpolyCommand::PaintFill(paint_fill::PaintFill { object_id: None, u: Some(0.5), v: Some(0.5), x: None, y: None }),
        LowpolyCommand::FillBucket(fill_bucket::FillBucket { object_id: None, u: Some(0.5), v: Some(0.5), x: None, y: None }),
        LowpolyCommand::TransformEnd(transform_end::TransformEnd {}),
        LowpolyCommand::ImportSnapshotJson(set_snapshot_json::ImportSnapshotJson { json: "{}".into() }),
        LowpolyCommand::SetFixtureJson(set_fixture_json::SetFixtureJson { json: "{}".into() }),
        LowpolyCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: Some("extrude".into()) }),
        LowpolyCommand::SetActiveObject(set_active_object::SetActiveObject { object_id: "obj-1".into() }),
        LowpolyCommand::SetActivePaintLayer(set_active_paint_layer::SetActivePaintLayer { layer_index: 0 }),
        LowpolyCommand::SetUtilityParam(set_utility_param::SetUtilityParam { key: "brushSize".into(), value_json: "20".into() }),
        LowpolyCommand::EngagementInput(engagement_input::EngagementInput { value: "ext".into() }),
        LowpolyCommand::ToggleShowEdges(toggle_show_edges::ToggleShowEdges {}),
        LowpolyCommand::ToggleSun(toggle_sun::ToggleSun {}),
        LowpolyCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: 45.0 }),
        LowpolyCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: 35.0 }),
        LowpolyCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: 0.8 }),
        LowpolyCommand::SetCamera(set_camera::SetCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 }),
        LowpolyCommand::PaintStrokeBegin(paint_stroke_begin::PaintStrokeBegin {}),
        LowpolyCommand::PaintSample(paint_sample::PaintSample { object_id: None, u: Some(0.5), v: Some(0.5), x: None, y: None }),
        LowpolyCommand::PaintStroke(paint_stroke::PaintStroke { object_id: None, u: Some(0.5), v: Some(0.5), x: None, y: None }),
        LowpolyCommand::PaintAt(paint_at::PaintAt { object_id: None, u: Some(0.5), v: Some(0.5), x: None, y: None }),
        LowpolyCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { object_id: None, u: None, v: None, x: Some(0.0), y: Some(0.0) }),
        LowpolyCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { object_id: None, u: None, v: None, x: Some(1.0), y: Some(1.0) }),
        LowpolyCommand::TransformBegin(transform_begin::TransformBegin {}),
    ]
}
//#endregion 🔖️CommandSurface

//#region 🔖️ManifestSanity
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let json = serde_json::to_string(&create_lowpoly_app()).expect("app definition json");
    for id in [edit::windows::model::LOWPOLY_PLAY_WINDOW_MAIN, paint_mode::windows::uv::LOWPOLY_PLAY_WINDOW_UV] {
        assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
    }
    for id in [edit::LOWPOLY_PLAY_MODE_EDIT, paint_mode::LOWPOLY_PLAY_MODE_PAINT, paint_mode::LOWPOLY_PLAY_LAYOUT_PAINT] {
        assert!(json.contains(id), "mode/layout {id} missing from the manifest");
    }
    for body in [LOWPOLY_PLAY_BODY_DOCUMENT, LOWPOLY_PLAY_BODY_CATALOGUE, LOWPOLY_PLAY_BODY_INSPECTION, LOWPOLY_PLAY_BODY_LAYERS] {
        assert!(json.contains(body), "panel body {body} missing from the manifest");
    }
    assert!(json.contains("3d.lowpoly"), "artifact kind missing from the manifest");
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the "mesh" domain is declared and
/// scoped to the Model window, and the framework auto-injects its six interaction actions.
#[semio_framework_async_macros::async_test]
async fn the_mesh_interaction_domain_is_declared_and_scoped_to_the_model_window() {
    let definition = create_lowpoly_app();
    let mesh = definition.interactions.iter().find(|interaction| interaction.id == MESH_INTERACTION_DOMAIN).expect("mesh domain declared");
    assert_eq!(mesh.granularities.iter().map(|granularity| granularity.id.as_str()).collect::<Vec<_>>(), vec!["object", "vertex", "edge", "face"]);
    assert!(matches!(mesh.hierarchy, HierarchyProvider::Flat));
    let main_window = definition.window_kinds.iter().find(|window| window.id == edit::windows::model::LOWPOLY_PLAY_WINDOW_MAIN).expect("main window declared");
    assert_eq!(main_window.interactions, vec![InteractionRef::new(MESH_INTERACTION_DOMAIN)]);
    for injected in ["interactionSelect", "interactionHover", "clearSelection", "selectAll", "setSelectionMode", "setInteractionGranularity"] {
        assert!(main_window.actions.iter().any(|action| action.id == injected), "framework must auto-inject {injected}");
    }
    for deleted in ["setSelection", "toggleSelectionKind", "toggleSelectionTarget", "setSelectionMethod", "setSelectionModeDefault", "worldSelect", "worldHover", "setHover", "worldPick"] {
        assert!(!definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == deleted), "{deleted} must no longer be app-declared");
    }
}
//#endregion 🔖️ManifestSanity

//#region 🔖️CrossCutting
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_edits_via_backbone() {
    testkit::assert_two_instances_converge::<EditorApp<LowpolyPlayApp>, _>(
        "mem://lowpoly-convergence",
        LowpolyCommand::PatchObject(patch_object::PatchObject { object_id: "obj-1".into(), field: "name".into(), value_json: Some(serde_json::to_string("Renamed By A").unwrap()) }),
        LowpolyCommand::AddPrimitive(add_primitive::AddPrimitive { kind: Some("box".into()) }),
        |app| app.snapshot().expect("projection"),
    )
    .await;
}

#[semio_framework_async_macros::async_test]
async fn ingest_operations_is_idempotent() {
    testkit::assert_ingest_idempotent::<EditorApp<LowpolyPlayApp>, _>(LowpolyCommand::PatchObject(patch_object::PatchObject { object_id: "obj-1".into(), field: "name".into(), value_json: Some(serde_json::to_string("Hero").unwrap()) }), |app| {
        app.snapshot().expect("projection")
    })
    .await;
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    use crate::editor::lowpoly::testkit::render;
    let mut a = app().await;
    assert!(render(&mut a, "lowpoly.play.nope").await.contains("Unknown body"));
}
//#endregion 🔖️CrossCutting

//#region 🔖️MediaPorts
#[semio_framework_async_macros::async_test]
async fn export_media_mesh_out_produces_mesh_document_payload() {
    let mut a: LowpolyApp = app().await;
    let media = semio_framework_plugin::resolve_ready(a.export_media("mesh:out")).expect("export mesh:out");
    assert_eq!(media.media_type, MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh });
    match media.payload {
        MediaPayload::Structured { schema, .. } => assert_eq!(schema, "mesh.document"),
        other => panic!("expected Structured payload, got {other:?}"),
    }
}

/// 🧬️ `"mesh:in"` replaces the whole document via `reset_document_effect` (a
/// `Effect::LoadDocument`, outside undo history) — whole-document replace has no replacement
/// mutation per `📓️taxonomy.md`, so this is an effect, not an `artifact_mutations` entry.
#[semio_framework_async_macros::async_test]
async fn import_media_mesh_in_round_trips_into_a_reset_document_effect() {
    let mesh = semio_framework_plugin::mesh_from_kind("box");
    let mesh_document = crate::schema::mesh_document_from_mesh(&mesh).expect("mesh document");
    let json = serde_json::to_string(&mesh_document).expect("mesh document json");
    let media = Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh }, payload: MediaPayload::Structured { schema: "mesh.document".into(), json } };
    let projection = crate::schema::default_snapshot();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&projection, &history);
    let emit = LowpolyPlayApp::import_media("mesh:in", &media, &doc).expect("import mesh:in");
    assert!(emit.artifact_mutations.is_empty(), "whole-document replace is an effect, not a mutation");
    let semio_framework_plugin::Effect::LoadDocument { pack, .. } = emit.effects.first().expect("mesh:in must emit a LoadDocument effect") else {
        panic!("expected a LoadDocument effect");
    };
    let loaded = <LowpolySnapshot as ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
    assert_eq!(loaded.objects.len(), 1);
}
//#endregion 🔖️MediaPorts

//#region 🔖️ContextMenuRegistry
#[semio_framework_async_macros::async_test]
async fn registry_wired_app_dispatches_add_primitive() {
    let mut a = app_with_registry().await;
    crate::editor::lowpoly::testkit::dispatch(&mut a, LowpolyCommand::AddPrimitive(add_primitive::AddPrimitive { kind: Some("plane".into()) })).await;
    assert_eq!(a.snapshot().expect("projection").objects.len(), 2);
}
//#endregion 🔖️ContextMenuRegistry
