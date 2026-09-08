
use super::*;
use crate::editor::animate::PresentationCommand;
use crate::editor::animate::testkit::{presentation_app, render as render_body};
use dsl::os_pack::json::Value;
use semio_framework_plugin::testkit::meta;

#[semio_framework_async_macros::async_test]
async fn renders_canvas_2d_scene() {
    let mut app = presentation_app().await;
    let rendered = render_body(&mut app, PRESENTATION_PLAY_BODY_MAIN).await;
    assert!(rendered.contains("canvas-2d") || rendered.contains("Canvas2d"));
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_canvas_2d_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, PRESENTATION_PLAY_BODY_MAIN);
    assert!(matches!(definition.surface_kind, SurfaceKind::Canvas2d));
    assert!(definition.options.measures.is_empty(), "animate presentation declares no live chrome measures");
}

#[semio_framework_async_macros::async_test]
async fn source_frame_renders_as_actual_image_layer_behind_tiles() {
    let mut app = presentation_app().await;
    app.dispatch_typed(PresentationCommand::SeedGrid(crate::editor::animate::commands::seed_grid::SeedGrid { rows: 1, columns: 2 }), &meta("local")).await.expect("seed grid");
    let deck = app.snapshot().expect("projection");
    let layers_json = deck_to_canvas_layers(&deck);
    let layers: Vec<Value> = dsl::os_pack::json::parse(&layers_json).unwrap().as_array().cloned().unwrap_or_default();
    let (source, _) = crate::presentation_working_scene(&deck);
    assert!(!source.src.trim().is_empty());
    let source_layer = layers.first().expect("source layer is first (renders behind tiles)");
    assert_eq!(source_layer.get("id").and_then(|v| v.as_str()), Some("source-frame"));
    assert_eq!(source_layer.get("kind").and_then(|v| v.as_str()), Some("image"));
    assert_eq!(source_layer.get("dataUrl").and_then(|v| v.as_str()), Some(source.src.as_str()));
    for tile_layer in &layers[1..] {
        assert_ne!(tile_layer.get("kind").and_then(|v| v.as_str()), Some("image"));
        assert!(tile_layer.get("dataUrl").is_none() || tile_layer.get("dataUrl") == Some(&Value::Null));
    }
}

#[semio_framework_async_macros::async_test]
async fn deck_to_canvas_layers_omits_data_url_when_source_has_no_image() {
    let base = crate::default_presentation_snapshot();
    let (mut source, tiles) = crate::presentation_working_scene(&base);
    source.src = String::new();
    let deck = crate::presentation_snapshot_with_tiles(&source, &tiles);
    let layers_json = deck_to_canvas_layers(&deck);
    let layers: Vec<Value> = dsl::os_pack::json::parse(&layers_json).unwrap().as_array().cloned().unwrap_or_default();
    let source_layer = layers.first().expect("source layer presentation");
    assert_eq!(source_layer.get("kind").and_then(|v| v.as_str()), Some("source"));
    assert!(source_layer.get("dataUrl").is_none() || source_layer.get("dataUrl") == Some(&Value::Null));
}

#[semio_framework_async_macros::async_test]
async fn deck_to_canvas_layers_treats_pdf_kind_as_non_image() {
    let base = crate::default_presentation_snapshot();
    let (mut source, tiles) = crate::presentation_working_scene(&base);
    source.kind = "pdf".into();
    let deck = crate::presentation_snapshot_with_tiles(&source, &tiles);
    let layers_json = deck_to_canvas_layers(&deck);
    let layers: Vec<Value> = dsl::os_pack::json::parse(&layers_json).unwrap().as_array().cloned().unwrap_or_default();
    let source_layer = layers.first().expect("source layer presentation");
    assert_eq!(source_layer.get("kind").and_then(|v| v.as_str()), Some("source"));
}
