//! 🖌️ Laws of the lowpoly paint tool: a press-drag-release is ONE `ToolTransaction` holding ONE `apply-paint-stroke`
//! whose dabs grew tick by tick — nothing reaches the document mid-stroke, the release commits one edit and one history
//! row stamped with the paint tool's `TransactionRef` and labelled from the leaf; a host abort leaves zero trace; a
//! click is a one-shot of one dab; the UV canvas strokes through the same tool; the eyedropper only samples.

use crate::editor::lowpoly::unit_tests::context::{act, app, committed_edits, dispatch, LowpolyApp};
use crate::editor::lowpoly::LowpolyCommand;
use semio_framework_plugin::PluginApp;

fn pixels(a: &LowpolyApp) -> Vec<u8> {
    a.snapshot().expect("projection").objects[0].paint_layers[0].materialized_pixels()
}

fn at(object_id: &str, u: f32, phase: Option<&str>) -> LowpolyCommand {
    LowpolyCommand::PaintAt(super::paint_at::PaintAt { object_id: Some(object_id.into()), u: Some(u), v: Some(0.5), x: None, y: None, phase: phase.map(str::to_string), reason: None })
}

/// 🧾️ The newest applied document row of the session history.
async fn last_row(a: &mut LowpolyApp) -> semio_framework::kernel::HistoryEntry {
    a.0.history_snapshot().await.expect("history snapshot").upserts.into_iter().filter(|entry| entry.kind == "mutation" && entry.applied).last().expect("a document row")
}

#[semio_framework_async_macros::async_test]
async fn add_paint_layer_emits_operation() {
    let mut a = app().await;
    let before = a.snapshot().expect("projection").objects[0].paint_layers.len();
    dispatch(&mut a, LowpolyCommand::AddPaintLayer(super::add_paint_layer::AddPaintLayer { object_id: None, name: Some("Detail".into()) })).await;
    assert_eq!(a.snapshot().expect("projection").objects[0].paint_layers.len(), before + 1);
}

/// 🖌️ Two streamed dabs and a release are ONE edit of ONE stroke leaf with both dabs, stamped with the paint tool's
/// transaction and labelled from the leaf; undo restores the exact pre-stroke pixels and redo repaints them.
#[semio_framework_async_macros::async_test]
async fn a_paint_stroke_is_one_edit_of_one_stroke_leaf() {
    let mut a = app().await;
    let object_id = a.snapshot().expect("projection").objects[0].id.clone();
    let before = pixels(&a);
    let edits_before = committed_edits(&mut a).await;
    dispatch(&mut a, at(&object_id, 0.5, Some("stream"))).await;
    dispatch(&mut a, at(&object_id, 0.52, Some("stream"))).await;
    assert_eq!(pixels(&a), before, "mid-stroke ticks commit nothing to the document");
    assert_eq!(committed_edits(&mut a).await, edits_before, "mid-stroke ticks are no edits");
    dispatch(&mut a, LowpolyCommand::PaintAt(super::paint_at::PaintAt { object_id: None, u: None, v: None, x: None, y: None, phase: Some("commit".into()), reason: None })).await;
    let painted = pixels(&a);
    assert_ne!(painted, before, "the stroke changed pixels");
    assert_eq!(committed_edits(&mut a).await, edits_before + 1, "the whole stroke commits as one document edit");
    let row = last_row(&mut a).await;
    let transaction = row.transaction.as_ref().expect("the row is keyed by its tool transaction");
    assert_eq!(transaction.tool, "s.lowpoly.lowpoly@1/*#editor#paint", "the paint tool authored it");
    assert_eq!(row.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En).to_string(), format!("Paint a stroke of 2 dabs on layer 0 of \"{object_id}\""));
    act(&mut a, "undo", serde_json::json!({})).await;
    assert_eq!(pixels(&a), before, "undo restores the exact pre-stroke pixels");
    act(&mut a, "redo", serde_json::json!({})).await;
    assert_eq!(pixels(&a), painted, "redo repaints the stroke");
}

/// 🧯️ A host abort drops the open stroke with zero trace: no edit, and the next click paints only its own dab.
#[semio_framework_async_macros::async_test]
async fn an_aborted_stroke_leaves_zero_trace() {
    let mut a = app().await;
    let object_id = a.snapshot().expect("projection").objects[0].id.clone();
    let before = pixels(&a);
    let edits_before = committed_edits(&mut a).await;
    dispatch(&mut a, at(&object_id, 0.3, Some("stream"))).await;
    dispatch(&mut a, at(&object_id, 0.35, Some("stream"))).await;
    dispatch(&mut a, LowpolyCommand::PaintAt(super::paint_at::PaintAt { object_id: None, u: None, v: None, x: None, y: None, phase: Some("abort".into()), reason: Some("blur".into()) })).await;
    assert_eq!(pixels(&a), before, "an aborted stroke paints nothing");
    assert_eq!(committed_edits(&mut a).await, edits_before, "an aborted stroke is no edit");
    dispatch(&mut a, at(&object_id, 0.7, None)).await;
    assert_eq!(committed_edits(&mut a).await, edits_before + 1, "a click is one one-shot edit");
    let row = last_row(&mut a).await;
    assert_eq!(row.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En).to_string(), format!("Paint a stroke of 1 dab on layer 0 of \"{object_id}\""), "the aborted dabs never reach a later gesture");
}

/// 🖼️ The UV canvas strokes through the same tool: a press opens the stroke, a move adds its samples, the release
/// commits ONE edit; a move with no open stroke leaves nothing.
#[semio_framework_async_macros::async_test]
async fn a_uv_canvas_stroke_is_one_edit_and_a_stray_move_is_nothing() {
    let mut a = app().await;
    let edits_before = committed_edits(&mut a).await;
    dispatch(&mut a, LowpolyCommand::CanvasPointerMove(super::canvas_pointer_move::CanvasPointerMove { object_id: None, u: None, v: None, x: Some(10.0), y: Some(10.0), samples: None })).await;
    assert_eq!(committed_edits(&mut a).await, edits_before, "a move without a press is nothing");
    dispatch(&mut a, LowpolyCommand::CanvasPointerDown(super::canvas_pointer_down::CanvasPointerDown { object_id: None, u: None, v: None, x: Some(0.0), y: Some(0.0) })).await;
    dispatch(&mut a, LowpolyCommand::CanvasPointerMove(super::canvas_pointer_move::CanvasPointerMove { object_id: None, u: None, v: None, x: Some(20.0), y: Some(0.0), samples: Some(vec![[10.0, 0.0], [20.0, 0.0]]) })).await;
    assert_eq!(committed_edits(&mut a).await, edits_before, "mid-stroke canvas moves are no edits");
    dispatch(&mut a, LowpolyCommand::CanvasPointerUp(super::canvas_pointer_up::CanvasPointerUp { cancelled: Some(false) })).await;
    assert_eq!(committed_edits(&mut a).await, edits_before + 1, "the release commits one edit");
    let row = last_row(&mut a).await;
    assert!(row.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En).to_string().starts_with("Paint a stroke of 3 dabs"), "the press dab and both samples: {:?}", row.label);
}

#[semio_framework_async_macros::async_test]
async fn eyedropper_updates_paint_color_without_operations() {
    let mut a = app().await;
    let before = a.snapshot().expect("projection");
    let edits_before = committed_edits(&mut a).await;
    dispatch(&mut a, LowpolyCommand::PaintSample(super::paint_sample::PaintSample { object_id: None, u: Some(0.5), v: Some(0.5), x: None, y: None })).await;
    assert_eq!(a.snapshot().expect("projection"), before, "sampling a colour edits nothing");
    assert_eq!(committed_edits(&mut a).await, edits_before, "sampling a colour commits no document edit");
}
