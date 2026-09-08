
use crate::editor::lowpoly::LowpolyCommand;
use crate::editor::lowpoly::testkit::{app, dispatch};

#[semio_framework_async_macros::async_test]
async fn set_active_object_is_view_state_and_emits_no_operations() {
    let mut a = app().await;
    let object_id = a.snapshot().expect("projection").objects[0].id.clone();
    let result = dispatch(&mut a, LowpolyCommand::SetActiveObject(super::set_active_object::SetActiveObject { object_id })).await;
    assert!(result.mutations.is_empty(), "setting the active object must not create an undoable operation");
}

#[semio_framework_async_macros::async_test]
async fn set_active_paint_layer_is_view_state_and_emits_no_operations() {
    let mut a = app().await;
    let result = dispatch(&mut a, LowpolyCommand::SetActivePaintLayer(super::set_active_paint_layer::SetActivePaintLayer { layer_index: 0 })).await;
    assert!(result.mutations.is_empty());
}
