use semio_framework_3d::mesh::{MeshModelingOps};

/// 🎟️ Unbounded retained grant: the pre-migration step budget carried no retained limit.
const UNBOUNDED_STEP_GRANT: semio_framework_job::RetainedCloneGrant = semio_framework_job::RetainedCloneGrant { maximum_items: usize::MAX, maximum_copy_bytes: usize::MAX, maximum_capacity_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: usize::MAX };
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
#[path = "../🧰️support/🦀️.rs"]
mod context;

use semio_s_artifact_lowpoly_lowpoly_editor::*;
use semio_s_artifact_lowpoly_lowpoly_commands::{paint::LowpolyPaintTick, paint::lowpoly_paint_tick};
use semio_s_artifact_lowpoly_lowpoly_view::editor::lowpoly::session::lowpoly_paint_drive;
use semio_s_artifact_lowpoly_lowpoly_mut_paint::{apply_paint_stroke::ApplyPaintStroke, edit_paint_layer::EditPaintLayer};
use semio_framework_tool_machine::GesturePhase;
use semio_framework_artifact_reference::io::text::artifact_reference::ArtifactReferenceText;
use semio_s_artifact_lowpoly_lowpoly_commands::{add_primitive, camera, chrome, engagement, document, media, mesh_edit, object, paint, patch_object, selection, sun, transform, utility, uv};
use semio_s_artifact_lowpoly_lowpoly_config::{LowpolyConfig, LowpolyConfigMutation, SetActiveObjectEdit, SetPaintUtilityEdit, SetUtilityParamsEdit, SetPaintColorEdit, SetEngagementInputEdit, SetSunEdit};
use semio_s_artifact_lowpoly_lowpoly_modes::{self, editor::lowpoly::modes::paint as paint_mode};
use semio_s_artifact_lowpoly_lowpoly_panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel, layers as layers_panel};
use semio_s_artifact_lowpoly_lowpoly_view::{editor::lowpoly::session::LowpolyScratch, editor::lowpoly::session::LowpolyTransient, editor::lowpoly::session::LowpolyTransientMutation};
use semio_s_artifact_lowpoly_lowpoly_view::editor::lowpoly::terminology::LowpolyLabels;
use semio_s_artifact_lowpoly_lowpoly_view::{resolve_active_object_id, selection_from_interaction, selection_from_state, utility_param_f64, LowpolyView, MESH_GRANULARITY_OBJECT, MESH_INTERACTION_DOMAIN};
use semio_s_artifact_lowpoly_lowpoly_mutations::LowpolyMutation;
use semio_s_artifact_lowpoly_lowpoly_model::{snapshot::artifact_kind, snapshot::LowpolyObject, snapshot::LowpolySnapshot, snapshot::LOWPOLY_DOCUMENT_SCHEMA};
use protocol::Mutation;
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_plugin::app::{ArtifactOwnedToolJobContext, InteractionView};
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::ActionRef;
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EphemeralEmit;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_ui_locale::LabelText;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPayload;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::UtilityCategory;
use semio_framework_plugin::UtilityDefinition;
use semio_framework_plugin::WindowEngagement;
use semio_framework_plugin::WindowEngagementInput;
use semio_framework_plugin::WindowEngagementOption;
use semio_framework_plugin::WindowEngagementPossible;
use semio_framework_plugin::WindowEngagementStatus;
use semio_framework_plugin::WindowMeasure;
use std::collections::HashMap;
use pack::snapshot::ArtifactPack;
use semio_framework_2d_compute::EngineHandles;
use camera::set_camera;
use chrome::toggle_show_edges;
use engagement::{engagement_input, engagement_submit};
use document::{replace_snapshot_json, set_snapshot_json};
use media::{export_mesh, import_mesh_file, load_mesh_request};
use object::{delete_selection, duplicate_object};
use mesh_edit::{bevel, decimate, dissolve, extrude, flip_faces, inset, loop_cut, merge, mirror, snap, subdivide, toggle_smooth, triangulate};
use paint::{add_paint_layer, canvas_pointer_down, canvas_pointer_move, canvas_pointer_up, fill_bucket, paint_at, paint_fill, paint_sample, paint_stroke};
use selection::{set_active_object, set_active_paint_layer};
use sun::{set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun};
use transform::{rotate_selection, scale_selection, translate_selection};
use utility::set_utility_param;
use uv::{clear_seam, mark_uv_seam, unwrap_active};
use semio_framework_plugin::InteractionRef;
use crate::{context::app, context::app_with_registry, context::dispatch, context::lowpoly_manifest_for_tests, context::LowpolyApp};
use semio_framework_plugin::{artifact_app_laws, EditorApp, PluginApp};

fn retained_operation() -> AppOperationContext {
    AppOperationContext { app_instance_id: 7, parent_document_id: "lowpoly-retained-test".into(), operation_id: 11, generation: 13, canonical_base_revision: [17; 32], authoring_seed: "authoring-seed-test".into() }
}

fn retained_context(transient: LowpolyTransient, transient_generation: u64) -> std::sync::Arc<ArtifactOwnedToolJobContext<EditorApp<LowpolyPlayApp>>> {
    std::sync::Arc::new(ArtifactOwnedToolJobContext::new(
        7,
        None,
        [17; 32],
        0,
        transient_generation,
        semio_framework_plugin::app::ArtifactOwnedToolJobSnapshots {
            children: std::sync::Arc::new(semio_framework_plugin::ChildContentView::EMPTY),
            draft: std::sync::Arc::new(NoDraft::default()),
            transient: std::sync::Arc::new(transient),
            window_config: None,
            window_transient: None,
        },
    ))
}

#[test]
fn retained_route_partition_and_publication_are_exact() {
    use semio_framework::{ToolCancellationPolicy, ToolExecutionShape};

    // 🎯️ The generated command schema is the one count every other table must match — 46 since
    // 2026-09-08 (`setActiveUtility` became framework-owned, `loadDocumentJson` became
    // `replaceSnapshotJson`); the literal 47 these assertions carried was never re-run.
    let declared = LowpolyCommand::TOOL_JOB_IDS.len();
    let all = every_command();
    let mut partition = LOWPOLY_MIGRATED_TOOL_IDS.to_vec();
    partition.sort_unstable();
    partition.dedup();
    assert_eq!(partition.len(), declared, "every generated command id is Migrated");
    assert_eq!(all.len(), partition.len());
    assert!(all.iter().all(|command| partition.binary_search(&command.command_id()).is_ok()));
    assert!(LOWPOLY_MIGRATED_TOOL_IDS.iter().all(|tool_id| lowpoly_command_disposition(tool_id).is_some()));
    assert_eq!(<LowpolyPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), declared);
    assert_eq!(<LowpolyCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.len(), declared);
    assert_eq!(lowpoly_contract().shape, ToolExecutionShape::Resumable);
    assert_eq!(lowpoly_contract().cancellation, ToolCancellationPolicy::PerOperation);
    assert_eq!((lowpoly_contract().checkpoint_every_steps, lowpoly_contract().progress_every_steps), (1, 1));
}

#[semio_framework_async_macros::async_test]
async fn retained_progress_replay_freshness_and_close_are_exact() {
    let command = LowpolyCommand::ToggleShowEdges(toggle_show_edges::ToggleShowEdges {});
    let snapshot = semio_s_artifact_lowpoly_lowpoly_model::standards::v1::subsets::any::io::text::snapshot::default_snapshot();
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
                snapshot_owner: None, command: &command,
                snapshot: &snapshot,
                config: &config,
                history: &history,
                interaction: &interaction,
                hover: &hover,
                context: Some(&context),
                operation: &operation
            }, &mut semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(256, u64::MAX, UNBOUNDED_STEP_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut 0, &mut semio_framework_job::RetainedCloneProgress::default()))
            .expect("progress"),
        ArtifactCommandWorkStep::Progress { .. }
    ));
    let checkpoint: Vec<u8> = (0..).map_while(|index| uninterrupted.checkpoint_byte(index)).collect();
    assert_eq!(checkpoint.len(), 72);
    let mut wrong_base = LowpolyRetainedCommandWork::new("toggleShowEdges", LowpolyCommandDisposition::Config, operation.operation_id, operation.generation, [18; 32], context_identity);
    assert!(wrong_base.restore(&checkpoint).is_err());
    let mut replayed = LowpolyRetainedCommandWork::new("toggleShowEdges", LowpolyCommandDisposition::Config, operation.operation_id, operation.generation, operation.canonical_base_revision, context_identity);
    replayed.restore(&checkpoint).expect("work restore");
    assert!(matches!(
        replayed
            .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs {
                snapshot_owner: None, command: &command,
                snapshot: &snapshot,
                config: &config,
                history: &history,
                interaction: &interaction,
                hover: &hover,
                context: Some(&context),
                operation: &operation
            }, &mut semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(256, u64::MAX, UNBOUNDED_STEP_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut 0, &mut semio_framework_job::RetainedCloneProgress::default()))
            .expect("replay"),
        ArtifactCommandWorkStep::Replay { .. }
    ));
    assert!(matches!(
        replayed
            .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs {
                snapshot_owner: None, command: &command,
                snapshot: &snapshot,
                config: &config,
                history: &history,
                interaction: &interaction,
                hover: &hover,
                context: Some(&context),
                operation: &operation
            }, &mut semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(256, u64::MAX, UNBOUNDED_STEP_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut 0, &mut semio_framework_job::RetainedCloneProgress::default()))
            .expect("complete"),
        ArtifactCommandWorkStep::Complete(_)
    ));
    let drifted = AppOperationContext { generation: operation.generation + 1, ..operation.clone() };
    let mut rejected = LowpolyRetainedCommandWork::new("toggleShowEdges", LowpolyCommandDisposition::Config, operation.operation_id, operation.generation, operation.canonical_base_revision, context_identity);
    assert!(rejected
        .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs { snapshot_owner: None, command: &command, snapshot: &snapshot, config: &config, history: &history, interaction: &interaction, hover: &hover, context: Some(&context), operation: &drifted }, &mut semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(256, u64::MAX, UNBOUNDED_STEP_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut 0, &mut semio_framework_job::RetainedCloneProgress::default()))
        .is_err());
    let drifted_context = retained_context(LowpolyTransient::default(), 20);
    assert!(rejected
        .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs {
            snapshot_owner: None, command: &command,
            snapshot: &snapshot,
            config: &config,
            history: &history,
            interaction: &interaction,
            hover: &hover,
            context: Some(&drifted_context),
            operation: &operation
        }, &mut semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(256, u64::MAX, UNBOUNDED_STEP_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut 0, &mut semio_framework_job::RetainedCloneProgress::default()))
        .is_err());
    assert!(matches!(
        rejected
            .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs {
                snapshot_owner: None, command: &command,
                snapshot: &snapshot,
                config: &config,
                history: &history,
                interaction: &interaction,
                hover: &hover,
                context: Some(&context),
                operation: &operation
            }, &mut semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(256, u64::MAX, UNBOUNDED_STEP_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut 0, &mut semio_framework_job::RetainedCloneProgress::default()))
            .expect("exact retry"),
        ArtifactCommandWorkStep::Progress { .. }
    ));
    assert_eq!(semio_framework_job::close_with_self_funded_context(close_grant(0), |cx| replayed.close_step(cx)).expect("original fixture close turn"), InteractiveJobCloseStep::Blocked);
    replayed.begin_close();
    assert_eq!(semio_framework_job::close_with_self_funded_context(close_grant(1), |cx| replayed.close_step(cx)).expect("original fixture close turn"), InteractiveJobCloseStep::Complete { progress: Default::default() });
    assert!(replayed.terminal_is_empty());
}

fn close_grant(maximum_items: usize) -> semio_framework_value::retained_clone::RetainedCloneGrant {
    semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 8 }
}

/// ⏱️ The interactive-step law over the UNIT BOX, not the concrete-forest default document: on 195
/// faces a debug build measures `mirror` at 42 ms, `loopCut` 18 ms, `toggleSmooth` 10 ms and even a
/// whole-object `translateSelection` 9 ms — the JSON parse + kernel pass + re-encode floor of one
/// monolithic mesh edit, which the runtime records as a single admitted overrun (`StepOverrunLedger`
/// quarantines only four CONSECUTIVE ones). Over the box every command's cost is its dispatch
/// overhead, which is what this law guards: `addPrimitive` re-encoding every untouched mesh (11.6 ms)
/// was such a bug (2026-09-18). Chunking whole-mesh kernel ops into resumable steps is a kernel job.
///
/// ⏱️ The ceiling is the runtime's own `INTERACTIVE_STEP_CEILING_US` in every build. What is measured is
/// the command's intrinsic turn cost: on a host oversubscribed three to four times (load 26–41 on ten
/// cores) a 5 ms turn takes 15–40 ms of wall time, so the law keeps the fastest of up to
/// `INTERACTIVE_TURN_ATTEMPTS` runs. A descheduled attempt is retried; a command whose own cost exceeds
/// the ceiling fails every attempt. Measured 2026-09-24 in a debug build at load 33: slowest command
/// `paintFill` 5.3 ms, every other command under 3.1 ms.
const INTERACTIVE_TURN_CEILING: std::time::Duration = std::time::Duration::from_micros(semio_framework_job::INTERACTIVE_STEP_CEILING_US);
const INTERACTIVE_TURN_ATTEMPTS: usize = 16;
#[semio_framework_async_macros::async_test]
async fn retained_migrated_turns_stay_below_eight_milliseconds() {
    let unit_box = semio_framework_3d::mesh::HalfedgeMesh::box_prim(1.0, 1.0, 1.0).expect("box prim").to_json().expect("box json");
    let snapshot = semio_s_artifact_lowpoly_lowpoly_model::standards::v1::subsets::any::io::text::snapshot::snapshot_from_mesh_json(&unit_box, "obj-1", "Unit Box");
    let config = LowpolyConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let history = HistoryView::empty();
    let operation = retained_operation();
    let context = retained_context(LowpolyTransient::default(), 29);
    for command in every_command().into_iter().filter(|command| LOWPOLY_MIGRATED_TOOL_IDS.contains(&command.command_id())) {
        let tool_id = command.command_id();
        let disposition = lowpoly_command_disposition(tool_id).expect("migrated disposition");
        let mut best = std::time::Duration::MAX;
        for _attempt in 0..INTERACTIVE_TURN_ATTEMPTS {
            let mut work = LowpolyRetainedCommandWork::new(tool_id, disposition, operation.operation_id, operation.generation, operation.canonical_base_revision, context.identity_digest());
            let mut slowest = std::time::Duration::ZERO;
            // 🔊️ Nothing is selected here, so a selection-bound mesh edit REFUSES (`no faces selected`) —
            // a refusal is a turn like any other and must respect the same ceiling.
            let mut refused = false;
            loop {
                let started = std::time::Instant::now();
                let step = work.step(&semio_framework_plugin::retained_command::ArtifactCommandInputs { snapshot_owner: None,
                    command: &command,
                    snapshot: &snapshot,
                    config: &config,
                    history: &history,
                    interaction: &interaction,
                    hover: &hover,
                    context: Some(&context),
                    operation: &operation,
                }, &mut semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(256, u64::MAX, UNBOUNDED_STEP_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut 0, &mut semio_framework_job::RetainedCloneProgress::default()));
                slowest = slowest.max(started.elapsed());
                match step {
                    Ok(ArtifactCommandWorkStep::Complete(_) | ArtifactCommandWorkStep::CompleteWithEphemeral { .. }) => break,
                    Ok(_) => {}
                    Err(fault) => {
                        // 🗿️ `deleteSelection` on the one-object box refuses "keeps at least one object" — that
                        // is its empty-selection answer at object granularity.
                        assert!(fault.message.contains("selected") || fault.message.contains("selection") || fault.message.contains("at least one object"), "{tool_id} refused for a reason other than the empty selection: {fault:?}");
                        refused = true;
                        break;
                    }
                }
            }
            best = best.min(slowest);
            if !refused {
                work.begin_close();
                assert_eq!(semio_framework_job::close_with_self_funded_context(close_grant(1), |cx| work.close_step(cx)).expect("original fixture close turn"), InteractiveJobCloseStep::Complete { progress: Default::default() });
            }
            if best < INTERACTIVE_TURN_CEILING {
                break;
            }
        }
        assert!(best < INTERACTIVE_TURN_CEILING, "{tool_id} turn exceeded {INTERACTIVE_TURN_CEILING:?} on every attempt: best {best:?}");
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
    assert_eq!(ids.len(), LowpolyCommand::TOOL_JOB_IDS.len(), "every LowpolyCommand row must be covered by every_command()");
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
        LowpolyCommand::TranslateSelection(translate_selection::TranslateSelection { dx: 1.0, dy: 0.0, dz: 0.0 }),
        LowpolyCommand::RotateSelection(rotate_selection::RotateSelection { ax: 0.0, ay: 1.0, az: 0.0, angle: 45.0 }),
        LowpolyCommand::ScaleSelection(scale_selection::ScaleSelection { sx: 1.0, sy: 1.0, sz: 1.0 }),
        LowpolyCommand::AddPaintLayer(add_paint_layer::AddPaintLayer { object_id: None, name: Some("Detail".into()) }),
        LowpolyCommand::PaintFill(paint_fill::PaintFill { object_id: None, u: Some(0.5), v: Some(0.5), x: None, y: None }),
        LowpolyCommand::FillBucket(fill_bucket::FillBucket { object_id: None, u: Some(0.5), v: Some(0.5), x: None, y: None }),
        LowpolyCommand::ImportSnapshotJson(set_snapshot_json::ImportSnapshotJson { json: "{}".into() }),
        LowpolyCommand::ReplaceSnapshotJson(replace_snapshot_json::ReplaceSnapshotJson { json: "{}".into() }),
        LowpolyCommand::ExportMesh(export_mesh::ExportMesh { format: "obj".into() }),
        LowpolyCommand::LoadMeshRequest(load_mesh_request::LoadMeshRequest {}),
        LowpolyCommand::ImportMeshFile(import_mesh_file::ImportMeshFile { name: "quad.obj".into(), payload: "v 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 3\n".into() }),
        LowpolyCommand::DeleteSelection(delete_selection::DeleteSelection {}),
        LowpolyCommand::DuplicateObject(duplicate_object::DuplicateObject { object_id: None }),
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
        LowpolyCommand::PaintSample(paint_sample::PaintSample { object_id: None, u: Some(0.5), v: Some(0.5), x: None, y: None }),
        LowpolyCommand::PaintStroke(paint_stroke::PaintStroke { object_id: None, u: Some(0.5), v: Some(0.5), x: None, y: None, phase: None, reason: None }),
        LowpolyCommand::PaintAt(paint_at::PaintAt { object_id: None, u: Some(0.5), v: Some(0.5), x: None, y: None, phase: None, reason: None }),
        LowpolyCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { object_id: None, u: None, v: None, x: Some(0.0), y: Some(0.0) }),
        LowpolyCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { object_id: None, u: None, v: None, x: Some(1.0), y: Some(1.0), samples: None }),
        LowpolyCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { cancelled: Some(false) }),
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
    for body in [LOWPOLY_PLAY_BODY_ARTIFACT, LOWPOLY_PLAY_BODY_CATALOGUE, LOWPOLY_PLAY_BODY_INSPECTION, LOWPOLY_PLAY_BODY_LAYERS] {
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
/// 🧹️ The REGISTERED pair: lowpoly publishes bounded tool proofs, so a registry-less `paired_apps`
/// instance faults in the `interactive-job.catalog-authority` proof join before any edit lands.
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_edits_via_backbone() {
    artifact_app_laws::assert_two_registered_instances_converge::<EditorApp<LowpolyPlayApp>, _, _, _>(
        "mem://lowpoly-convergence",
        || async { lowpoly_manifest_for_tests() },
        LowpolyCommand::PatchObject(patch_object::PatchObject { object_id: "obj-1".into(), field: "name".into(), value_json: Some(serde_json::to_string("Renamed By A").unwrap()) }),
        LowpolyCommand::AddPrimitive(add_primitive::AddPrimitive { kind: Some("box".into()) }),
        |app| app.snapshot().expect("projection"),semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()
    )
    .await;
}

/// 🔁️ `artifact_app_laws::assert_ingest_idempotent` over THIS crate's harness: the framework's registered
/// twin binds no live instance, so a tool-proof app refuses its very first typed command
/// (`interactive-job.live-instance`). Same law, same shape — a sender on a memory backbone, its envelopes
/// replayed twice onto a fresh receiver.
#[semio_framework_async_macros::async_test]
async fn ingest_operations_is_idempotent() {
    use store::{os_store::backbone::Backbone, os_store::backbone::BackboneMessage, os_store::backbone::MemoryBackbone};
    let mut sender = app().await;
    let (near, mut far) = MemoryBackbone::pair("mem://lowpoly-idempotent", "mem://lowpoly-idempotent").await;
    sender.attach_backbone(store::os_store::backbone::Backbones::Memory(near)).await.expect("attach sender");
    dispatch(&mut sender, LowpolyCommand::PatchObject(patch_object::PatchObject { object_id: "obj-1".into(), field: "name".into(), value_json: Some(serde_json::to_string("Hero").unwrap()) })).await;
    assert_eq!(sender.snapshot().expect("projection").objects[0].name, "Hero");
    let mut envelopes = Vec::new();
    for message in far.receive().await.expect("receive") {
        if let BackboneMessage::Mutations { envelopes: operations } = message {
            envelopes.extend(protocol::decode_envelopes(&operations).expect("decode envelopes"));
        }
    }
    assert!(!envelopes.is_empty(), "the rename reached the backbone");
    let operations = protocol::encode_envelopes(&envelopes);
    let mut receiver = app().await;
    receiver.0.ingest_operations(&operations, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), receiver.1.cx()).await.expect("ingest once");
    let once = receiver.snapshot().expect("projection");
    assert_eq!(once.objects[0].name, "Hero", "the replayed rename applies");
    receiver.0.ingest_operations(&operations, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), receiver.1.cx()).await.expect("ingest twice");
    assert_eq!(receiver.snapshot().expect("projection"), once, "feeding the same operation twice must not double-apply");
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    use crate::context::render;
    let mut a = app().await;
    assert!(render(&mut a, "lowpoly.play.nope").await.contains("Unknown body"));
}
//#endregion 🔖️CrossCutting

//#region 🔖️MediaPorts
#[semio_framework_async_macros::async_test]
async fn export_media_mesh_out_produces_mesh_document_payload() {
    let mut a: LowpolyApp = app().await;
    let media = ::semio_framework_async::poll::resolve_ready(semio_framework_plugin::artifact_app_laws::export_fixture_media(&mut a.0, "mesh:out", a.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close)).expect("export mesh:out");
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
    let mesh_document = semio_s_artifact_lowpoly_lowpoly_model::mesh_document_value(&mesh);
    let json = semio_framework_pack_json::to_json_string(&mesh_document);
    let media = Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh }, payload: MediaPayload::Structured { schema: "mesh.document".into(), json } };
    let projection = semio_s_artifact_lowpoly_lowpoly_model::standards::v1::subsets::any::io::text::snapshot::default_snapshot();
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
    dispatch(&mut a, LowpolyCommand::AddPrimitive(add_primitive::AddPrimitive { kind: Some("plane".into()) })).await;
    assert_eq!(a.snapshot().expect("projection").objects.len(), 2);
}
//#endregion 🔖️ContextMenuRegistry
