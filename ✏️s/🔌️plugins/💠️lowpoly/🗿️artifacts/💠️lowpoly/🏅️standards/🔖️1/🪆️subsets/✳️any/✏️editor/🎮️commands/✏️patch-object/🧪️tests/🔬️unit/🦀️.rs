use super::*;
use crate::editor::lowpoly::testkit::{app, dispatch};
use crate::editor::lowpoly::LowpolyCommand;

#[semio_framework_async_macros::async_test]
async fn patch_object_name_emits_operation() {
    let mut a = app().await;
    let object_id = a.snapshot().expect("projection").objects[0].id.clone();
    dispatch(&mut a, LowpolyCommand::PatchObject(PatchObject { object_id, field: "name".into(), value_json: Some(serde_json::to_string("Renamed").unwrap()) })).await;
    assert_eq!(a.snapshot().expect("projection").objects[0].name, "Renamed");
}
