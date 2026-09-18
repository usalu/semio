use crate::editor::lowpoly::unit_tests::context::{act, app, committed_edits, dispatch};
use crate::editor::lowpoly::LowpolyCommand;

#[semio_framework_async_macros::async_test]
async fn add_paint_layer_emits_operation() {
    let mut a = app().await;
    let before = a.snapshot().expect("projection").objects[0].paint_layers.len();
    dispatch(&mut a, LowpolyCommand::AddPaintLayer(super::add_paint_layer::AddPaintLayer { object_id: None, name: Some("Detail".into()) })).await;
    assert_eq!(a.snapshot().expect("projection").objects[0].paint_layers.len(), before + 1);
}

#[semio_framework_async_macros::async_test]
async fn paint_stroke_drag_is_one_undo_step_with_pixel_restoration() {
    let mut a = app().await;
    let object_id = a.snapshot().expect("projection").objects[0].id.clone();
    let before = a.snapshot().expect("projection").objects[0].paint_layers[0].pixels.clone();
    let edits_before = committed_edits(&mut a).await;
    // begin → tick → tick → end : one undoable PaintStroke edit.
    dispatch(&mut a, LowpolyCommand::PaintStrokeBegin(super::paint_stroke_begin::PaintStrokeBegin {})).await;
    dispatch(&mut a, LowpolyCommand::PaintAt(super::paint_at::PaintAt { object_id: Some(object_id.clone()), u: Some(0.5), v: Some(0.5), x: None, y: None })).await;
    dispatch(&mut a, LowpolyCommand::PaintAt(super::paint_at::PaintAt { object_id: Some(object_id), u: Some(0.52), v: Some(0.5), x: None, y: None })).await;
    assert_eq!(a.snapshot().expect("projection").objects[0].paint_layers[0].pixels, before, "mid-drag ticks commit nothing to the document");
    dispatch(&mut a, LowpolyCommand::PaintStrokeEnd(super::paint_stroke_end::PaintStrokeEnd {})).await;
    let painted = a.snapshot().expect("projection").objects[0].paint_layers[0].pixels.clone();
    assert_ne!(painted, before, "the stroke changed pixels");
    assert_eq!(committed_edits(&mut a).await, edits_before + 1, "the whole drag commits as one document edit");
    act(&mut a, "undo", serde_json::json!({})).await;
    let restored = a.snapshot().expect("projection").objects[0].paint_layers[0].pixels.clone();
    assert_eq!(restored, before, "undo restores the exact pre-stroke pixels");
    act(&mut a, "redo", serde_json::json!({})).await;
    assert_eq!(a.snapshot().expect("projection").objects[0].paint_layers[0].pixels, painted);
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
