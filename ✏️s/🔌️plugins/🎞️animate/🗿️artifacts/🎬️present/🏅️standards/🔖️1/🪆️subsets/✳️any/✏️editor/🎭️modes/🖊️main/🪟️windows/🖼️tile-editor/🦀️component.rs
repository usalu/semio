//! 🖼️ Animate present app — the tile-editor window: the canvas 2d surface rendering the source figure
//! backdrop plus its crop tiles.

use crate::artifacts::present::{FigureTileFrame, PresentSnapshot};
use semio_framework_plugin::{LocalizedLabel, SurfaceKind, WindowKindDefinition, WindowOptions};
use semio_framework_ui_contract::{surface, Buildable, BuiltNode, HasBase};
use semio_framework_ui_scene::{encode, Canvas2dScene};

//#region 🔖️Constants
pub const PRESENT_PLAY_WINDOW_MAIN: &str = "tile-editor";
pub const PRESENT_PLAY_BODY_MAIN: &str = "animate.present.play.main";
const PRESENT_PLAY_SURFACE_MAIN: &str = "animate.present.play";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::animate::create_animate_present_app`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: PRESENT_PLAY_WINDOW_MAIN.into(),
        label: LocalizedLabel::native("Tile editor", "Kacheleditor"),
        body_key: PRESENT_PLAY_BODY_MAIN.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "grid-3x3".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
        interactions: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️CanvasLayers
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct TileCanvasLayer {
    id: String,
    kind: String,
    name: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    /// 🖼️ Image src for `kind: "image"` layers, rendered by both the React and wgpu canvas-2d hosts.
    #[serde(skip_serializing_if = "Option::is_none")]
    data_url: Option<String>,
}

fn frame_to_canvas(frame: &FigureTileFrame, scale: f64) -> (f64, f64, f64, f64) {
    (frame.x * scale, frame.y * scale, frame.width * scale, frame.height * scale)
}

/// 🖼️ Renders the actual source figure (image) as the backdrop layer, with crop tiles drawn on top of
/// it. `config` no longer carries `selected_ids` (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-
/// MECHANISM: selection is framework-owned state, and `ArtifactApp::render` is never given an
/// `InteractionView`) — the selection overlay this used to bake into every tile's `kind` is gone; the
/// client renders that highlight itself from the framework's own interaction state now (matches
/// `🖍️draw`'s canvas render, same reason).
fn deck_to_canvas_layers(deck: &PresentSnapshot) -> String {
    const SCALE: f64 = 1000.0;
    let (source, tiles) = crate::artifacts::present::present_working_scene(deck);
    let mut layers = Vec::new();
    let (sx, sy, sw, sh) = frame_to_canvas(&source.frame, SCALE);
    let has_image_src = !source.src.trim().is_empty() && source.kind != "pdf";
    layers.push(TileCanvasLayer { id: "source-frame".into(), kind: if has_image_src { "image".into() } else { "source".into() }, name: source.src.clone(), x: sx, y: sy, width: sw, height: sh, data_url: has_image_src.then(|| source.src.clone()) });
    for tile in &tiles {
        let (x, y, width, height) = frame_to_canvas(&tile.crop, SCALE);
        layers.push(TileCanvasLayer { id: tile.id.clone(), kind: "tile".into(), name: tile.name.clone(), x, y, width, height, data_url: None });
    }
    serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into())
}
//#endregion 🔖️CanvasLayers

//#region 🔖️Render
pub fn render(deck: &PresentSnapshot) -> BuiltNode {
    let scene = Canvas2dScene { camera_x: 0.0, camera_y: 0.0, zoom: 1.0, layers_json: deck_to_canvas_layers(deck) };
    surface(encode(semio_framework_ui_contract::SurfaceKind::Canvas2d, &scene)).id(PRESENT_PLAY_SURFACE_MAIN).build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::animate::testkit::{present_app, render as render_body};
    use crate::editor::animate::PresentCommand;
    use semio_framework_plugin::testkit::meta;
    use serde_json::Value;

    #[semio_framework_async_macros::async_test]
    async fn renders_canvas_2d_scene() {
        let mut app = present_app().await;
        let rendered = render_body(&mut app, PRESENT_PLAY_BODY_MAIN).await;
        assert!(rendered.contains("canvas-2d") || rendered.contains("Canvas2d"));
    }

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_canvas_2d_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, PRESENT_PLAY_BODY_MAIN);
        assert!(matches!(definition.surface_kind, SurfaceKind::Canvas2d));
        assert!(definition.options.measures.is_empty(), "animate present declares no live chrome measures");
    }

    #[semio_framework_async_macros::async_test]
    async fn source_frame_renders_as_actual_image_layer_behind_tiles() {
        let mut app = present_app().await;
        app.dispatch_typed(PresentCommand::SeedGrid(crate::editor::animate::commands::seed_grid::SeedGrid { rows: 1, columns: 2 }), &meta("local")).await.expect("seed grid");
        let deck = app.snapshot().await.expect("projection");
        let layers_json = deck_to_canvas_layers(&deck);
        let layers: Vec<Value> = serde_json::from_str(&layers_json).unwrap();
        let (source, _) = crate::artifacts::present::present_working_scene(&deck);
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
        let base = crate::artifacts::present::default_present_snapshot();
        let (mut source, tiles) = crate::artifacts::present::present_working_scene(&base);
        source.src = String::new();
        let deck = crate::artifacts::present::present_snapshot_with_tiles(&source, &tiles);
        let layers_json = deck_to_canvas_layers(&deck);
        let layers: Vec<Value> = serde_json::from_str(&layers_json).unwrap();
        let source_layer = layers.first().expect("source layer present");
        assert_eq!(source_layer.get("kind").and_then(|v| v.as_str()), Some("source"));
        assert!(source_layer.get("dataUrl").is_none() || source_layer.get("dataUrl") == Some(&Value::Null));
    }

    #[semio_framework_async_macros::async_test]
    async fn deck_to_canvas_layers_treats_pdf_kind_as_non_image() {
        let base = crate::artifacts::present::default_present_snapshot();
        let (mut source, tiles) = crate::artifacts::present::present_working_scene(&base);
        source.kind = "pdf".into();
        let deck = crate::artifacts::present::present_snapshot_with_tiles(&source, &tiles);
        let layers_json = deck_to_canvas_layers(&deck);
        let layers: Vec<Value> = serde_json::from_str(&layers_json).unwrap();
        let source_layer = layers.first().expect("source layer present");
        assert_eq!(source_layer.get("kind").and_then(|v| v.as_str()), Some("source"));
    }
}
//#endregion 🧪️Tests
