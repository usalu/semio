//! 🖌️ Laws of the input window's brush tool: one stroke is ONE `ToolTransaction` holding ONE parametric
//! `paint-input-stroke` leaf — one edit, one history row stamped with its `TransactionRef`, labelled from the leaf in
//! English and German — whether it arrives in one dispatch or streams over many through the window transient; a
//! cancel leaves zero trace; two strokes are two transactions; and a stroke edited in history replays its downstream
//! exactly like a fresh fold of the edited log.

use super::*;
use crate::editor::bitmap::modes::edit::windows::input::transient::BitmapInputWindowTransient;
use crate::editor::bitmap::modes::edit::windows::input::WFC_BITMAP_WINDOW_INPUT;
use crate::editor::bitmap::{BitmapEditor, BitmapEditorCommand};
use crate::mutations::{paint_input_stroke, set_input_pixels, BitmapMutation, BitmapStrokePoint};
use crate::schema::snapshot::encode_base64;
use crate::{BitmapSnapshot, WFC_BITMAP_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ActionMeta, App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};

fn point(x: u32, y: u32) -> BitmapStrokePoint {
    BitmapStrokePoint { x, y }
}

fn base() -> BitmapSnapshot {
    crate::tests::fixtures::base()
}

fn request(points: &[(u32, u32)], color: u32) -> BrushToolRequest {
    BrushToolRequest::on(&base(), points.iter().map(|(x, y)| point(*x, *y)).collect(), color)
}

const SEED: &str = "authoring-seed-brush";

//#region 🛠️Tool
#[test]
fn the_phase_protocol_reads_every_host_spelling() {
    assert_eq!(BitmapStrokePhase::parse(None, None), Some(BitmapStrokePhase::Once));
    assert_eq!(BitmapStrokePhase::parse(Some("stream"), None), Some(BitmapStrokePhase::Stream));
    assert_eq!(BitmapStrokePhase::parse(Some("commit"), None), Some(BitmapStrokePhase::Commit));
    assert_eq!(BitmapStrokePhase::parse(Some("abort"), Some("blur")), Some(BitmapStrokePhase::Abort(ToolAbortReason::Blur)));
    assert_eq!(BitmapStrokePhase::parse(Some("abort"), None), Some(BitmapStrokePhase::Abort(ToolAbortReason::Tool)));
    assert_eq!(BitmapStrokePhase::parse(Some("sideways"), None), None);
}

#[test]
fn a_one_shot_stroke_is_one_committed_transaction_of_one_leaf() {
    let (committed, transient) = bitmap_brush_dispatch(BitmapStrokePhase::Once, request(&[(0, 0), (3, 2)], 1), &BitmapInputWindowTransient::default(), SEED);
    let (reference, mutations) = committed.expect("the stroke commits");
    assert!(reference.id.starts_with("tx-"), "{reference:?}");
    assert_eq!(reference.tool, "s.wfc.bitmap@1/*#editor#paint-stroke");
    assert_eq!(mutations, vec![paint_input_stroke(vec![point(0, 0), point(3, 2)], 1)]);
    assert!(transient.brush.is_none(), "a one-shot leaves the window at rest");
}

#[test]
fn a_streamed_stroke_is_one_transaction_across_dispatches() {
    let (none, open) = bitmap_brush_dispatch(BitmapStrokePhase::Stream, request(&[(0, 0)], 1), &BitmapInputWindowTransient::default(), SEED);
    assert!(none.is_none(), "a tick publishes nothing");
    let first = open.brush.as_ref().expect("the window holds the open stroke").transaction.clone();
    let (none, open) = bitmap_brush_dispatch(BitmapStrokePhase::Stream, request(&[(2, 0), (2, 2)], 0), &open, SEED);
    assert!(none.is_none());
    assert_eq!(open.brush.as_ref().expect("still open").transaction, first, "every tick joins the transaction minted at the first");
    let (committed, rest) = bitmap_brush_dispatch(BitmapStrokePhase::Commit, request(&[(3, 2)], 0), &open, SEED);
    let (reference, mutations) = committed.expect("the release commits");
    assert_eq!(reference, first, "the commit publishes under the ref minted at the first tick");
    assert_eq!(mutations, vec![paint_input_stroke(vec![point(0, 0), point(2, 0), point(2, 2), point(3, 2)], 1)], "ONE net leaf in the colour the stroke opened with");
    assert!(rest.brush.is_none());
}

#[test]
fn every_host_abort_leaves_zero_trace() {
    for reason in [ToolAbortReason::Blur, ToolAbortReason::CaptureLost, ToolAbortReason::Frozen, ToolAbortReason::Retired, ToolAbortReason::Tool] {
        let (_, open) = bitmap_brush_dispatch(BitmapStrokePhase::Stream, request(&[(0, 0), (1, 1)], 1), &BitmapInputWindowTransient::default(), SEED);
        assert!(open.brush.is_some());
        let (committed, rest) = bitmap_brush_dispatch(BitmapStrokePhase::Abort(reason), request(&[], 1), &open, SEED);
        assert!(committed.is_none() && rest.brush.is_none(), "{reason:?} must leave nothing");
    }
    let (committed, rest) = bitmap_brush_dispatch(BitmapStrokePhase::Abort(ToolAbortReason::Blur), request(&[], 1), &BitmapInputWindowTransient::default(), SEED);
    assert!(committed.is_none() && rest == BitmapInputWindowTransient::default(), "an abort at rest is a silent no-op");
}

#[test]
fn two_strokes_are_two_transactions() {
    let (first, _) = bitmap_brush_dispatch(BitmapStrokePhase::Once, request(&[(0, 0)], 1), &BitmapInputWindowTransient::default(), "seed-one");
    let (second, _) = bitmap_brush_dispatch(BitmapStrokePhase::Once, request(&[(1, 1)], 1), &BitmapInputWindowTransient::default(), "seed-two");
    assert_ne!(first.expect("first commits").0, second.expect("second commits").0);
}

#[test]
fn a_stroke_that_paints_no_cell_leaves_zero_trace() {
    let (committed, rest) = bitmap_brush_dispatch(BitmapStrokePhase::Once, request(&[(9, 9), (12, 9)], 1), &BitmapInputWindowTransient::default(), SEED);
    assert!(committed.is_none() && rest.brush.is_none());
    let (_, open) = bitmap_brush_dispatch(BitmapStrokePhase::Stream, request(&[(9, 9)], 1), &BitmapInputWindowTransient::default(), SEED);
    assert!(open.brush.is_some(), "a stream may start outside the sample and enter it later");
    let (committed, rest) = bitmap_brush_dispatch(BitmapStrokePhase::Commit, request(&[(10, 9)], 1), &open, SEED);
    assert!(committed.is_none() && rest.brush.is_none(), "a released stroke that never entered the sample commits nothing");
}

#[test]
fn a_one_shot_interrupts_an_open_stroke() {
    let (_, open) = bitmap_brush_dispatch(BitmapStrokePhase::Stream, request(&[(0, 0)], 1), &BitmapInputWindowTransient::default(), SEED);
    let (committed, rest) = bitmap_brush_dispatch(BitmapStrokePhase::Once, request(&[(3, 0)], 1), &open, SEED);
    assert_eq!(committed.expect("the one-shot commits").1, vec![paint_input_stroke(vec![point(3, 0)], 1)], "the interrupted stroke contributes nothing");
    assert!(rest.brush.is_none());
}

#[test]
fn a_tampered_persisted_stroke_is_dropped_with_zero_trace() {
    let (_, mut open) = bitmap_brush_dispatch(BitmapStrokePhase::Stream, request(&[(0, 0)], 1), &BitmapInputWindowTransient::default(), SEED);
    open.brush.as_mut().expect("open").states = vec!["no-such-state".to_string()];
    let (committed, rest) = bitmap_brush_dispatch(BitmapStrokePhase::Commit, request(&[(1, 0)], 1), &open, SEED);
    assert_eq!(committed.expect("the commit runs from rest as a one-shot").1, vec![paint_input_stroke(vec![point(1, 0)], 1)]);
    assert!(rest.brush.is_none());
}

#[test]
fn the_window_previews_committed_plus_the_open_stroke() {
    let (_, open) = bitmap_brush_dispatch(BitmapStrokePhase::Stream, request(&[(0, 0), (3, 2)], 1), &BitmapInputWindowTransient::default(), SEED);
    let preview = bitmap_brush_preview(&base(), &open).expect("an open stroke previews");
    let mut expected = base();
    crate::mutations::apply_bitmap_mutation(&mut expected, &paint_input_stroke(vec![point(0, 0), point(3, 2)], 1)).expect("the leaf applies");
    assert_eq!(preview, expected);
    assert!(bitmap_brush_preview(&base(), &BitmapInputWindowTransient::default()).is_none(), "a resting window previews nothing");
}

#[test]
fn the_brush_state_round_trips_the_window_transient_codecs() {
    let (_, open) = bitmap_brush_dispatch(BitmapStrokePhase::Stream, request(&[(0, 0), (1, 2)], 1), &BitmapInputWindowTransient::default(), SEED);
    let text = <BitmapInputWindowTransient as store::ArtifactDsl>::print_dsl(&open);
    assert_eq!(<BitmapInputWindowTransient as store::ArtifactDsl>::parse_dsl(&text).expect("text parses"), open);
    let packed = <BitmapInputWindowTransient as store::ArtifactPack>::encode_pack(&open);
    assert_eq!(<BitmapInputWindowTransient as store::ArtifactPack>::decode_pack(&packed).expect("pack decodes"), open);
}
//#endregion 🛠️Tool

//#region 🖱️Pointer
#[test]
fn a_hover_a_stray_release_and_a_secondary_press_mean_nothing_to_the_brush() {
    let (width, height) = (base().input.width, base().input.height);
    assert_eq!(bitmap_brush_pointer_stroke(&BitmapBrushPointer::Move { samples: vec![[1.5, 1.5]] }, width, height, false), None);
    assert_eq!(bitmap_brush_pointer_stroke(&BitmapBrushPointer::Up { world: Some([1.0, 1.0]), cancelled: false }, width, height, false), None);
    assert_eq!(bitmap_brush_pointer_stroke(&BitmapBrushPointer::Up { world: None, cancelled: true }, width, height, false), None);
    assert_eq!(bitmap_brush_pointer_stroke(&BitmapBrushPointer::Down { world: Some([1.0, 1.0]), button: 2 }, width, height, false), None);
    assert_eq!(bitmap_brush_pointer_stroke(&BitmapBrushPointer::Down { world: Some([f64::NAN, 1.0]), button: 0 }, width, height, false), None);
}

#[test]
fn pointer_samples_map_to_clamped_cells_with_repeats_collapsed() {
    let (width, height) = (base().input.width, base().input.height);
    let down = bitmap_brush_pointer_stroke(&BitmapBrushPointer::Down { world: Some([0.9, 0.1]), button: 0 }, width, height, true).expect("a primary press opens");
    assert_eq!((down.phase, down.points, down.interrupt), (BitmapStrokePhase::Stream, vec![point(0, 0)], true), "a press drops a stroke a lost release left open");
    let drag = bitmap_brush_pointer_stroke(&BitmapBrushPointer::Move { samples: vec![[0.2, 0.2], [1.2, 0.4], [1.8, 0.6], [-3.0, -2.0], [99.0, 1.0]] }, width, height, true).expect("a drag streams");
    assert_eq!(drag.points, vec![point(0, 0), point(1, 0), point(0, 0), point(width - 1, 1)], "repeats collapse and a drag past an edge clamps onto it");
    let release = bitmap_brush_pointer_stroke(&BitmapBrushPointer::Up { world: Some([2.5, 1.5]), cancelled: false }, width, height, true).expect("a release commits");
    assert_eq!((release.phase, release.points), (BitmapStrokePhase::Commit, vec![point(2, 1)]));
    let cancel = bitmap_brush_pointer_stroke(&BitmapBrushPointer::Up { world: None, cancelled: true }, width, height, true).expect("a cancel aborts");
    assert_eq!(cancel.phase, BitmapStrokePhase::Abort(ToolAbortReason::CaptureLost));
}

#[test]
fn a_streamed_stroke_never_repeats_a_cell_across_ticks() {
    let (_, open) = bitmap_brush_dispatch(BitmapStrokePhase::Stream, request(&[(1, 1)], 1), &BitmapInputWindowTransient::default(), SEED);
    let (_, open) = bitmap_brush_dispatch(BitmapStrokePhase::Stream, request(&[(1, 1), (1, 1), (2, 1)], 1), &open, SEED);
    let (committed, _) = bitmap_brush_dispatch(BitmapStrokePhase::Commit, request(&[(2, 1)], 1), &open, SEED);
    assert_eq!(committed.expect("the release commits").1, vec![paint_input_stroke(vec![point(1, 1), point(2, 1)], 1)]);
}
//#endregion 🖱️Pointer

//#region 🧩️MountedApp
type BitmapApp = VcsArtifactApp<EditorApp<BitmapEditor>>;

fn block_on<F: std::future::Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let mut context = std::task::Context::from_waker(std::task::Waker::noop());
    loop {
        match future.as_mut().poll(&mut context) {
            std::task::Poll::Ready(output) => return output,
            std::task::Poll::Pending => std::thread::yield_now(),
        }
    }
}

fn manifest() -> App {
    App { definition: crate::editor::bitmap::create_bitmap_editor(), examples: Vec::new() }
}

fn app() -> BitmapApp {
    let mut app = block_on(semio_framework_plugin::artifact_app_laws::new_app_with_registry::<EditorApp<BitmapEditor>>(manifest));
    block_on(app.bind_instance_id(1));
    app
}

fn input_meta() -> ActionMeta {
    let window_instances = vec![ViewWindowInstance { id: WFC_BITMAP_WINDOW_INPUT.into(), window_kind_id: WFC_BITMAP_WINDOW_INPUT.into() }];
    let view = ViewModel { window_instances, ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) }.for_window_instance(WFC_BITMAP_WINDOW_INPUT).expect("the input window is in the roster");
    ActionMeta { view_state: Some(view), ..semio_framework_plugin::artifact_app_laws::meta("local") }
}

fn settle(app: &mut BitmapApp, result: Result<InvocationResult, semio_framework_plugin::Fault>) -> InvocationResult {
    let mut result = result.expect("the dispatch is admitted");
    for _ in 0..1_048_576 {
        while let Some(page) = app.take_typed_operation_result_page(1) {
            assert!(page.lane != semio_framework_plugin::app::TypedOperationResultLane::Fault, "{}", String::from_utf8_lossy(page.bytes()));
            app.acknowledge_typed_operation_result(page.token).expect("the page is acknowledged");
        }
        while let Some(completion) = block_on(app.take_typed_operation_completion()).expect("the completion is taken") {
            if let Some(patch) = completion.history_patch {
                match result.history_patch.as_mut() {
                    Some(previous) => previous.upserts.extend(patch.upserts),
                    None => result.history_patch = Some(patch),
                }
            }
        }
        while app.take_typed_operation_effect().is_some() || app.take_typed_operation_event().is_some() {}
        let _ = app.take_typed_operation_ui_scope();
        if !app.has_pending_typed_operations() {
            return result;
        }
        PluginApp::maintenance_step(app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("maintenance");
        block_on(app.advance_typed_operation_publication()).expect("publication advances");
    }
    panic!("the bitmap dispatch did not settle");
}

fn stroke(app: &mut BitmapApp, points: &[(u32, u32)], phase: Option<&str>) -> InvocationResult {
    let command = BitmapEditorCommand::PaintStroke { xs: points.iter().map(|point| point.0).collect(), ys: points.iter().map(|point| point.1).collect(), color: Some(1), phase: phase.map(str::to_string), reason: None };
    let meta = input_meta();
    let result = block_on(app.dispatch_typed(command, &meta));
    settle(app, result)
}

fn edit_rows(result: &InvocationResult) -> Vec<semio_framework::kernel::HistoryEntry> {
    result.history_patch.as_ref().map(|patch| patch.upserts.iter().filter(|entry| entry.applied && !entry.op_lines.is_empty()).cloned().collect()).unwrap_or_default()
}

fn close(app: &mut BitmapApp) {
    for _ in 0..1_048_576 {
        if app.close_terminal_is_empty() {
            return;
        }
        if PluginApp::close_step(app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("bitmap app close") == semio_framework_plugin::PluginCloseStep::Complete {
            break;
        }
    }
    assert!(app.close_terminal_is_empty(), "the bitmap app did not reach terminal-empty ownership");
}

#[test]
fn one_mounted_stroke_is_one_edit_one_row_and_one_transaction() {
    let mut app = app();
    let before = app.snapshot().expect("projection");
    let result = stroke(&mut app, &[(0, 0), (3, 3)], None);
    let rows = edit_rows(&result);
    assert_eq!(rows.len(), 1, "one stroke, one history row: {rows:?}");
    let transaction = rows[0].transaction.as_ref().expect("the row is keyed by its tool transaction");
    assert!(transaction.id.starts_with("tx-") && transaction.tool == "s.wfc.bitmap@1/*#editor#paint-stroke", "{transaction:?}");
    assert!(rows[0].op_lines.iter().all(|line| line.starts_with("paint-input-stroke")), "the op is the parametric leaf: {:?}", rows[0].op_lines);
    assert_eq!(rows[0].label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), "Paint stroke of 4 cells in colour 1");
    assert_eq!(rows[0].label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De), "Strich mit 4 Zellen in Farbe 1 malen");
    assert_ne!(app.snapshot().expect("projection"), before, "the stroke landed");
    close(&mut app);
}

#[test]
fn a_mounted_streamed_stroke_commits_once_and_a_cancelled_one_leaves_zero_trace() {
    let mut app = app();
    let before = app.snapshot().expect("projection");
    assert!(edit_rows(&stroke(&mut app, &[(0, 0)], Some("stream"))).is_empty(), "a tick is no history");
    assert!(edit_rows(&stroke(&mut app, &[(2, 2)], Some("stream"))).is_empty());
    assert_eq!(app.snapshot().expect("projection"), before, "ticks never touch the document");
    let rows = edit_rows(&stroke(&mut app, &[(4, 0)], Some("commit")));
    assert_eq!(rows.len(), 1, "the release is ONE row: {rows:?}");
    assert!(rows[0].transaction.is_some());
    let committed = app.snapshot().expect("projection");
    assert!(edit_rows(&stroke(&mut app, &[(5, 5), (6, 6)], Some("stream"))).is_empty());
    let meta = input_meta();
    let abort = BitmapEditorCommand::PaintStroke { xs: Vec::new(), ys: Vec::new(), color: None, phase: Some("abort".into()), reason: Some("blur".into()) };
    let result = block_on(app.dispatch_typed(abort, &meta));
    assert!(edit_rows(&settle(&mut app, result)).is_empty(), "a cancel is no history");
    assert_eq!(app.snapshot().expect("projection"), committed, "a cancel leaves zero trace");
    close(&mut app);
}

#[test]
fn two_mounted_strokes_are_two_transactions() {
    let mut app = app();
    let first = edit_rows(&stroke(&mut app, &[(0, 0), (1, 0)], None));
    let second = edit_rows(&stroke(&mut app, &[(0, 2), (1, 2)], None));
    let (first, second) = (first[0].transaction.clone().expect("first ref"), second[0].transaction.clone().expect("second ref"));
    assert_ne!(first.id, second.id);
    close(&mut app);
}

/// 🖱️ Dispatches one canvas verb exactly as both hosts send it (surface id as the React host stamps it, world
/// coordinates on every command) and settles it.
fn pointer(app: &mut BitmapApp, action: &str, args: serde_json::Value) -> InvocationResult {
    let mut args = args;
    args["surfaceId"] = serde_json::Value::from(format!("window:{WFC_BITMAP_WINDOW_INPUT}"));
    let command = <BitmapEditor as semio_framework_plugin::ArtifactEditor>::command_from_action(action, Some(&dsl::DslValue::from(&args))).expect("the canvas verb bridges");
    let meta = input_meta();
    let result = block_on(app.dispatch_typed(command, &meta));
    settle(app, result)
}

#[test]
fn a_mounted_press_drag_release_paints_one_stroke_and_a_hover_or_a_cancel_leaves_zero_trace() {
    let mut app = app();
    let before = app.snapshot().expect("projection");
    assert!(edit_rows(&pointer(&mut app, "canvasPointerMove", serde_json::json!({ "worldSamples": [[1.5, 1.5]] }))).is_empty(), "a hover is no stroke");
    assert!(edit_rows(&pointer(&mut app, "canvasPointerUp", serde_json::json!({ "worldX": 1.5, "worldY": 1.5, "cancelled": false }))).is_empty(), "a release with nothing open is no stroke");
    assert_eq!(app.snapshot().expect("projection"), before);
    assert!(edit_rows(&pointer(&mut app, "canvasPointerDown", serde_json::json!({ "worldX": 0.25, "worldY": 0.75, "button": 0 }))).is_empty(), "a press opens the stroke, no history");
    assert!(edit_rows(&pointer(&mut app, "canvasPointerMove", serde_json::json!({ "worldSamples": [[0.5, 0.5], [1.5, 0.5], [2.5, 0.5]] }))).is_empty());
    assert_eq!(app.snapshot().expect("projection"), before, "ticks never touch the document");
    let rows = edit_rows(&pointer(&mut app, "canvasPointerUp", serde_json::json!({ "worldX": 2.9, "worldY": 1.1, "cancelled": false })));
    assert_eq!(rows.len(), 1, "press, drag, release is ONE row: {rows:?}");
    assert!(rows[0].transaction.as_ref().is_some_and(|transaction| transaction.tool == "s.wfc.bitmap@1/*#editor#paint-stroke"));
    assert!(rows[0].op_lines.iter().all(|line| line.starts_with("paint-input-stroke")), "{:?}", rows[0].op_lines);
    let committed = app.snapshot().expect("projection");
    assert_ne!(committed, before, "the stroke landed");
    assert!(edit_rows(&pointer(&mut app, "canvasPointerDown", serde_json::json!({ "worldX": 5.0, "worldY": 5.0, "button": 0 }))).is_empty());
    assert!(edit_rows(&pointer(&mut app, "canvasPointerUp", serde_json::json!({ "worldX": 6.0, "worldY": 6.0, "cancelled": true }))).is_empty(), "a cancelled release is no history");
    assert_eq!(app.snapshot().expect("projection"), committed, "a cancelled stroke leaves zero trace");
    close(&mut app);
}
//#endregion 🧩️MountedApp

//#region ⏪️TimeTravel
/// ⏪️ Time travel edits a stroke's inputs, never the brush: a stroke superseded with another colour previews as the
/// state before it plus the draft, and its Report replay re-applies the downstream region write onto the edited
/// stroke — exactly the fresh fold of the edited log.
#[test]
fn a_stroke_edited_in_history_replays_its_downstream() {
    use protocol::OpBinary;
    block_on(async {
        let mut store = store::ArtifactStore::<BitmapSnapshot, BitmapMutation>::new(store::create_document_envelope::<BitmapSnapshot, BitmapMutation>(WFC_BITMAP_DOCUMENT_SCHEMA, "brush-time-travel", base(), None)).await.expect("the store opens");
        store.install_document_store_owners_exact(semio_framework_plugin::bounded_document_store_owners::<BitmapSnapshot, BitmapMutation>());
        let log = [paint_input_stroke(vec![point(0, 0), point(3, 2)], 1), set_input_pixels(0, 0, 1, 1, encode_base64(&[0]))];
        for mutation in &log {
            store.dispatch(store::ArtifactCommand::Apply { mutations: vec![mutation.clone()], description: None, transaction: None }).await.expect("the edit applies");
        }
        let ids: Vec<protocol::MutationId> = store.mutation_ops().expect("applied operations").into_iter().map(|operation| operation.mutation_id).collect();
        let edited = paint_input_stroke(vec![point(0, 0), point(3, 2)], 0);
        let drafts: std::collections::BTreeMap<protocol::MutationId, protocol::InputReplacement> = [(ids[0].clone(), protocol::InputReplacement::Input { schema: WFC_BITMAP_DOCUMENT_SCHEMA.into(), payload: edited.encode_op().expect("the edited leaf encodes") })].into_iter().collect();
        let mut preview = store.state_before(&ids[0], &drafts).expect("the preview base folds").as_ref().clone();
        assert_eq!(preview, base(), "the preview base is the state right before the edited stroke");
        crate::mutations::apply_bitmap_mutation(&mut preview, &edited).expect("the draft applies to its base");
        let mut replay = store.begin_report_replay(&drafts, Some(&ids[0])).expect("the replay begins at the edited stroke");
        assert!(matches!(replay.step(store.replay_edits(), &mut || false).expect("the replay steps"), store::ReplayStep::Finished(_)));
        let result = replay.finish().expect("a finished replay yields its result");
        let report = store.replay_report(&result).expect("report");
        assert!(!report.blocks_finalize(), "a recoloured stroke never blocks finalizing");
        let mut fresh = base();
        for mutation in [edited, log[1].clone()] {
            let _ = crate::mutations::apply_bitmap_mutation(&mut fresh, &mutation);
        }
        assert_eq!(result.state().expect("the replay reached a state").as_ref(), &fresh, "the replay equals the fresh fold of the edited log");
        store.commit_finished_replay(result, store::HistoryFinalization::Overwrite).await.expect("overwrite commits");
        assert_eq!(store.snapshot_ref(), &fresh, "the overwritten history folds to the edited state");
        let mut disposer = semio_framework_plugin::bounded_document_store_disposer::<BitmapSnapshot, BitmapMutation>();
        for _ in 0..4_096 {
            if disposer.terminal_is_empty(&store) {
                break;
            }
            disposer.close_step(&mut store, 1, 64 * 1024).expect("the store retires");
        }
        assert!(disposer.terminal_is_empty(&store), "the standalone store retires to its terminal-empty shell");
    });
}
//#endregion ⏪️TimeTravel
