use super::*;
use crate::editor::fem2d::commands::set_active_example;
use crate::editor::fem2d::modes::edit::windows::model as model_window;
use crate::editor::fem2d::unit_tests::context::{dispatch, fem2d_app, render};
use crate::editor::fem2d::Fem2dCommand;
use semio_framework_plugin::artifact_app_laws::meta;
use semio_framework_plugin::{PluginApp, ViewModel, ViewWindowInstance};

#[semio_framework_async_macros::async_test]
async fn focus_entity_centres_the_model_window_camera_on_the_entity() {
    let mut app = fem2d_app();
    dispatch(&mut app, Fem2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: crate::examples::demo::ID.into() })).await;
    let before = app.snapshot().expect("snapshot");
    let result = dispatch(&mut app, Fem2dCommand::FocusEntity(FocusEntity { id: "ridge".into() })).await;
    assert!(result.mutations.is_empty(), "framing the camera is window state, never a document mutation");
    assert_eq!(app.snapshot().expect("snapshot"), before);
    let scene: semio_framework_plugin::Canvas2dScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene(&render(&mut app, model_window::BODY_KEY)).expect("model scene");
    assert_eq!((scene.camera_x, scene.camera_y), model_window::screen_2d(4.0, 7.6), "the camera centres on the ridge node's layer-space point");
    assert!((scene.zoom - 1.0).abs() < 1e-9, "focusing keeps the window's own zoom");

    dispatch(&mut app, Fem2dCommand::FocusEntity(FocusEntity { id: "r1".into() })).await;
    let scene: semio_framework_plugin::Canvas2dScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene(&render(&mut app, model_window::BODY_KEY)).expect("model scene");
    assert_eq!((scene.camera_x, scene.camera_y), model_window::screen_2d(11.0, 2.8), "a region frames on its outline centroid");
}

#[semio_framework_async_macros::async_test]
async fn focus_entity_refuses_an_unknown_id() {
    let mut app = fem2d_app();
    dispatch(&mut app, Fem2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: crate::examples::demo::ID.into() })).await;
    let mut action = meta("local");
    action.view_state = Some(ViewModel {
        window_id: Some("model-left".into()),
        window_instances: vec![ViewWindowInstance { id: "model-left".into(), window_kind_id: crate::editor::fem2d::modes::edit::windows::model::WINDOW_KIND_ID.into() }],
        ..Default::default()
    });
    let outcome = app.dispatch_typed(Fem2dCommand::FocusEntity(FocusEntity { id: "not-an-entity".into() }), &action).await;
    assert!(outcome.is_err(), "an unaddressable entity must fault rather than silently move the camera");
}
