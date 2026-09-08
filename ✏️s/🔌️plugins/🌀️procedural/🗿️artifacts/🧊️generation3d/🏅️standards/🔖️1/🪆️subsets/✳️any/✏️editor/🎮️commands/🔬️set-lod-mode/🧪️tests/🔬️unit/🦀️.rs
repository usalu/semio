
use super::*;
use crate::editor::generation3d::Generation3dCommand;
use crate::editor::generation3d::commands::set_active_utility;
use crate::editor::generation3d::testkit::{app, app_with_registry, dispatch};

#[semio_framework_async_macros::async_test]
async fn set_lod_mode_is_a_view_action_with_no_artifact_mutations() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut app = app().await;
    let before = app.snapshot().expect("snapshot");
    dispatch(&mut app, Generation3dCommand::SetLodMode(SetLodMode { value: "wireframe".into() })).await;
    assert_eq!(app.snapshot().expect("snapshot"), before, "setLodMode must not mutate the document");
}

#[semio_framework_async_macros::async_test]
async fn set_active_utility_switch_clears_scratch_and_emits_no_operations() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut app = app_with_registry().await;
    let before = app.snapshot().expect("snapshot");
    let result = app.dispatch_typed(Generation3dCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: "rotate".into() }), &semio_framework_plugin::testkit::meta("local")).await.expect("switch utility");
    assert!(result.mutations.is_empty(), "utility switching never emits document operations");
    assert_eq!(app.snapshot().expect("snapshot"), before, "utility switching records no history entry");
}
