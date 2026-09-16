use super::*;
use crate::editor::gis2d::terminology::gis2d_labels;
use crate::editor::gis2d::unit_tests::context::{app, close, main_window_measures, render as render_body, render_tiled_map_scene, Gis2dApp};
use crate::editor::gis2d::{GIS2D_FEATURE_GRANULARITY, GIS2D_INTERACTION_DOMAIN};
use semio_framework_plugin::artifact_app_laws::meta;
use semio_framework_plugin::{PluginApp, INTERACTION_HOVER_ACTION_ID, INTERACTION_SELECT_ACTION_ID};

/// 🕹️ Dispatches one framework-injected interaction verb against the `"features"` domain with the
/// exact wire payload `TiledMapHost` sends (`emitFeatureSelection`/the hover effect), then settles
/// the reserved job it spawns so the next render reads the published interaction state.
async fn interact(app: &mut Gis2dApp, action: &str, args: serde_json::Value) {
    let admitted = app.handle_action(action, Some(&dsl::DslValue::from(&args)), &meta("local")).await.unwrap_or_else(|error| panic!("{action} admission: {error:?}"));
    semio_framework_plugin::app::settle_framework_reserved_admission(app, admitted).await.unwrap_or_else(|error| panic!("{action} settles its reserved job: {error:?}"));
}

fn feature_targets(ids: &[&str]) -> String {
    let targets: Vec<serde_json::Value> = ids.iter().map(|id| serde_json::json!({ "granularity": GIS2D_FEATURE_GRANULARITY, "id": id })).collect();
    serde_json::to_string(&targets).expect("targets json")
}

/// 🕹️ The selection law: after `TiledMapHost`'s `interactionSelect` lands on a feature id, the very
/// next rendered scene marks that feature selected — the round trip that silently failed while the
/// host dispatched the deleted `setFeatureSelection` verb and the window never read the domain.
#[semio_framework_async_macros::async_test]
async fn an_interaction_select_marks_the_feature_selected_in_the_rendered_scene() {
    let mut app = app().await;
    let position_id = app.snapshot().expect("projection").positions.first().expect("fixture position").id.clone();
    let before = render_tiled_map_scene(&mut app, GIS2D_PLAY_BODY_COMPOSITE).await;
    assert_eq!(before.selection_json, r#"{"positions":[],"routes":[]}"#, "a fresh map has nothing selected");
    drop(before);
    interact(&mut app, INTERACTION_SELECT_ACTION_ID, serde_json::json!({ "domainId": GIS2D_INTERACTION_DOMAIN, "targets": feature_targets(&[position_id.as_str()]), "merge": "replace", "method": "pick" })).await;
    let after = render_tiled_map_scene(&mut app, GIS2D_PLAY_BODY_COMPOSITE).await;
    assert_eq!(after.selection_json, format!(r#"{{"positions":["{position_id}"],"routes":[]}}"#), "the picked position is marked selected on the surface the host repaints from");
    drop(after);
    close(&mut app);
}

/// 🐁️ The hover law: `interactionHover` on the `"pointer"` channel reaches the scene as the
/// `{kind, id}` record `TiledMapHost`'s popup reads.
#[semio_framework_async_macros::async_test]
async fn an_interaction_hover_reaches_the_scene_as_the_popup_record() {
    let mut app = app().await;
    let route_id = app.snapshot().expect("projection").routes.first().expect("fixture route").id.clone();
    let before = render_tiled_map_scene(&mut app, GIS2D_PLAY_BODY_COMPOSITE).await;
    assert_eq!(before.hover_json, "null", "a fresh map hovers nothing");
    drop(before);
    interact(&mut app, INTERACTION_HOVER_ACTION_ID, serde_json::json!({ "domainId": GIS2D_INTERACTION_DOMAIN, "channel": "pointer", "targets": feature_targets(&[route_id.as_str()]) })).await;
    let after = render_tiled_map_scene(&mut app, GIS2D_PLAY_BODY_COMPOSITE).await;
    assert_eq!(serde_json::from_str::<serde_json::Value>(&after.hover_json).expect("hover json"), serde_json::json!({ "kind": "route", "id": route_id }), "the hovered route reaches the popup lane with its own kind");
    drop(after);
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn renders_gis_map_scene() {
    let mut app = app().await;
    assert!(render_body(&mut app, GIS2D_PLAY_BODY_COMPOSITE).await.contains("tiled-map"));
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn render_canvas_uses_absolute_tile_urls_when_env_set() {
    unsafe { std::env::set_var("SEMIO_ASSET_BASE_URL", "http://127.0.0.1:6141") };
    let mut app = app().await;
    let scene = render_tiled_map_scene(&mut app, GIS2D_PLAY_BODY_COMPOSITE).await;
    assert_eq!(scene.tile_url_template, "http://127.0.0.1:6141/osm/{z}/{x}/{y}.png");
    assert_eq!(scene.vector_tile_url_template, "http://127.0.0.1:6141/vt/{z}/{x}/{y}.pbf");
    unsafe { std::env::remove_var("SEMIO_ASSET_BASE_URL") };
    drop(scene);
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn the_window_collects_every_option_node_exactly_once() {
    let config = MapWindowConfig::default();
    let measures = window_measures(&config, gis2d_labels(&semio_framework_plugin::ViewModel::default()));
    assert_eq!(measures.len(), 5, "3 selects + the layers and layer-weights groups");
    let mut app = app().await;
    assert_eq!(main_window_measures(&mut app).await.len(), measures.len(), "the app routes the same set under the window id");
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn the_definition_binds_the_tiled_map_surface_to_the_composite_body() {
    let definition = definition();
    assert_eq!(definition.id, GIS2D_PLAY_WINDOW_MAIN);
    assert_eq!(definition.body_key, GIS2D_PLAY_BODY_COMPOSITE);
    assert!(matches!(definition.surface_kind, SurfaceKind::TiledMap));
    assert!(definition.options.measures.is_empty(), "measures are config-derived per frame, never frozen into the manifest");
}
