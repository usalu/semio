use super::*;
use crate::editor::animate::unit_tests::context::{presentation_app, render as render_body};
use crate::editor::animate::PresentationCommand;
use dsl::os_pack::json::Value;
use semio_framework_plugin::artifact_app_laws::meta;

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
    crate::editor::animate::unit_tests::context::dispatch(&mut app, PresentationCommand::SeedGrid(crate::editor::animate::commands::seed_grid::SeedGrid { rows: 1, columns: 2 })).await;
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

/// 🎥️ LAW: the first-paint camera frames the whole deck. A Canvas2d camera names the view centre
/// (`Canvas2dHost`'s `worldToScreen`: `(world - camera) * zoom + viewport / 2`), so every layer of the
/// default and the `demo` deck must land inside the narrowest pane the play grid measures (469 px
/// square) without panning — the fixed `(0, 0, 1)` camera started the deck at the pane's centre and
/// clipped the rest.
#[test]
fn the_first_paint_camera_frames_every_layer_inside_the_narrowest_pane() {
    const PANE: f64 = 469.0;
    for deck in [crate::default_presentation_snapshot(), crate::demo_presentation_snapshot()] {
        let node = render(&deck).expect("tile editor canvas");
        let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("canvas surface") };
        let scene: Canvas2dScene = semio_framework_ui_scene::decode(props).expect("packed canvas");
        assert!(scene.zoom > 0.0 && scene.zoom <= 1.0, "zoom {} must fit, never magnify", scene.zoom);
        let layers: Vec<Value> = dsl::os_pack::json::parse(&scene.layers_json).expect("layers JSON").as_array().cloned().unwrap_or_default();
        assert!(!layers.is_empty());
        for layer in &layers {
            let field = |key: &str| layer.get(key).and_then(|value| value.as_f64()).expect("numeric layer bound");
            let screen = |world: f64, camera: f64| (world - camera) * scene.zoom + PANE * 0.5;
            let (left, top) = (screen(field("x"), scene.camera_x), screen(field("y"), scene.camera_y));
            let (right, bottom) = (screen(field("x") + field("width"), scene.camera_x), screen(field("y") + field("height"), scene.camera_y));
            assert!(left >= 0.0 && top >= 0.0 && right <= PANE && bottom <= PANE, "layer {:?} lands at [{left}, {top}]–[{right}, {bottom}] outside a {PANE} px pane", layer.get("id"));
        }
    }
}
