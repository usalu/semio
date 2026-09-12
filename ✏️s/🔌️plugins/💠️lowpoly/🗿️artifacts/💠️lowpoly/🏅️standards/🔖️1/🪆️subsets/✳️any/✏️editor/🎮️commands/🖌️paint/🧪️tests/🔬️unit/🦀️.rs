use crate::editor::lowpoly::unit_tests::context::{app, dispatch};
use crate::editor::lowpoly::LowpolyCommand;
use semio_framework_plugin::{artifact_app_laws, PluginApp};

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
    // begin → tick → tick → end : one undoable PaintStroke edit.
    a.dispatch_typed(LowpolyCommand::PaintStrokeBegin(super::paint_stroke_begin::PaintStrokeBegin {}), &artifact_app_laws::meta("a")).await.unwrap();
    let tick_a = a.dispatch_typed(LowpolyCommand::PaintAt(super::paint_at::PaintAt { object_id: Some(object_id.clone()), u: Some(0.5), v: Some(0.5), x: None, y: None }), &artifact_app_laws::meta("a")).await.unwrap();
    let tick_b = a.dispatch_typed(LowpolyCommand::PaintAt(super::paint_at::PaintAt { object_id: Some(object_id), u: Some(0.52), v: Some(0.5), x: None, y: None }), &artifact_app_laws::meta("a")).await.unwrap();
    assert!(tick_a.mutations.is_empty() && tick_b.mutations.is_empty(), "mid-drag ticks emit no operations");
    let end = a.dispatch_typed(LowpolyCommand::PaintStrokeEnd(super::paint_stroke_end::PaintStrokeEnd {}), &artifact_app_laws::meta("a")).await.unwrap();
    assert_eq!(end.mutations.len(), 1, "the whole drag commits as one operation");
    let painted = a.snapshot().expect("projection").objects[0].paint_layers[0].pixels.clone();
    assert_ne!(painted, before, "the stroke changed pixels");
    a.handle_action("undo", None, &artifact_app_laws::meta("a")).await.unwrap();
    let restored = a.snapshot().expect("projection").objects[0].paint_layers[0].pixels.clone();
    assert_eq!(restored, before, "undo restores the exact pre-stroke pixels");
    a.handle_action("redo", None, &artifact_app_laws::meta("a")).await.unwrap();
    assert_eq!(a.snapshot().expect("projection").objects[0].paint_layers[0].pixels, painted);
}

#[semio_framework_async_macros::async_test]
async fn eyedropper_updates_paint_color_without_operations() {
    let mut a = app().await;
    let result = a.dispatch_typed(LowpolyCommand::PaintSample(super::paint_sample::PaintSample { object_id: None, u: Some(0.5), v: Some(0.5), x: None, y: None }), &artifact_app_laws::meta("a")).await.unwrap();
    assert!(result.mutations.is_empty());
}
