use super::*;
use crate::editor::gis2d::modes::edit::windows::map::GIS2D_PLAY_BODY_COMPOSITE;
use crate::editor::gis2d::testkit::{app, close, dispatch, render};
use crate::editor::gis2d::Gis2dCommand;

#[semio_framework_async_macros::async_test]
async fn set_render_mode_is_view_state() {
    let mut app = app().await;
    let result = dispatch(&mut app, Gis2dCommand::SetRenderMode(set_render_mode::SetRenderMode { value: "vector".into() })).await;
    assert_eq!(result.artifact_publication_count(), 0);
    assert!(render(&mut app, GIS2D_PLAY_BODY_COMPOSITE).await.contains("\"renderMode\":\"vector\""));
    drop(result);
    close(&mut app);
}

/// 👁️ A representative View action mutates only config state, so under the real registry it
/// emits no operations and never trips the View → emits-operations guard.
#[semio_framework_async_macros::async_test]
async fn view_actions_emit_no_ops_under_registry_kind_discipline() {
    let mut app = app().await;
    let render_mode = dispatch(&mut app, Gis2dCommand::SetRenderMode(set_render_mode::SetRenderMode { value: "vector".into() })).await;
    assert_eq!(render_mode.artifact_publication_count(), 0, "render mode is config state");
    let fit = dispatch(&mut app, Gis2dCommand::FitWorld(fit_world::FitWorld {})).await;
    assert_eq!(fit.artifact_publication_count(), 0, "framing the world only moves the config camera");
    drop(render_mode);
    drop(fit);
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn toggling_a_layer_flips_its_visibility_in_the_rendered_scene() {
    let mut app = app().await;
    assert!(render(&mut app, GIS2D_PLAY_BODY_COMPOSITE).await.contains("\\\"water\\\":true"));
    dispatch(&mut app, Gis2dCommand::ToggleLayerVisibility(toggle_layer_visibility::ToggleLayerVisibility { layer_id: "water".into() })).await;
    assert!(render(&mut app, GIS2D_PLAY_BODY_COMPOSITE).await.contains("\\\"water\\\":false"));
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn layer_stroke_scale_is_clamped_to_the_surface_crates_range() {
    let mut app = app().await;
    dispatch(&mut app, Gis2dCommand::SetLayerStrokeScale(set_layer_stroke_scale::SetLayerStrokeScale { layer_id: "roads".into(), value: 99.0 })).await;
    let json = render(&mut app, GIS2D_PLAY_BODY_COMPOSITE).await;
    assert!(!json.contains("\\\"roads\\\":99"), "an out-of-range weight is clamped before it reaches the config");
    drop(json);
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn camera_writes_straight_through_to_the_config() {
    let mut app = app().await;
    dispatch(&mut app, Gis2dCommand::SetCamera(set_camera::SetCamera { camera_json: r#"{"x":5,"y":6,"zoom":7}"#.into() })).await;
    let json = render(&mut app, GIS2D_PLAY_BODY_COMPOSITE).await;
    assert!(json.contains("\\\"zoom\\\":7"));
    drop(json);
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn focus_feature_on_an_unknown_id_emits_nothing() {
    let mut app = app().await;
    let result = dispatch(&mut app, Gis2dCommand::FocusFeature(focus_feature::FocusFeature { feature_id: "nope".into(), feature_kind: "position".into() })).await;
    assert_eq!(result.artifact_publication_count(), 0);
    drop(result);
    close(&mut app);
}
