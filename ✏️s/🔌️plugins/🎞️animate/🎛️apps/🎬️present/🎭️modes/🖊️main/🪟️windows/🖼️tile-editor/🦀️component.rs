//! 🖼️ Animate present app — the tile-editor window: the canvas 2d surface rendering the source figure
//! backdrop plus its crop tiles.

use crate::apps::present::PRESENT_PLAY_APP_ID;
use crate::artifacts::present::{FigureTileFrame, PresentSnapshot};
use semio_framework_plugin::{build_canvas_2d_scene, Canvas2dScene, LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const PRESENT_PLAY_WINDOW_MAIN: &str = "tile-editor";
pub const PRESENT_PLAY_BODY_MAIN: &str = "animate.present.play.main";
const PRESENT_PLAY_SURFACE_MAIN: &str = "animate.present.play";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::apps::present::create_animate_present_app`.
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

/// 🖼️ Renders the actual source figure (image) as the backdrop layer, with crop tiles drawn on top of it.
fn deck_to_canvas_layers(deck: &PresentSnapshot, selected: &[String]) -> String {
    const SCALE: f64 = 1000.0;
    let mut layers = Vec::new();
    let (sx, sy, sw, sh) = frame_to_canvas(&deck.source.frame, SCALE);
    let has_image_src = !deck.source.src.trim().is_empty() && deck.source.kind != "pdf";
    layers.push(TileCanvasLayer {
        id: "source-frame".into(),
        kind: if has_image_src { "image".into() } else { "source".into() },
        name: deck.source.src.clone(),
        x: sx,
        y: sy,
        width: sw,
        height: sh,
        data_url: has_image_src.then(|| deck.source.src.clone()),
    });
    for tile in &deck.tiles {
        let (x, y, width, height) = frame_to_canvas(&tile.crop, SCALE);
        let selected_flag = selected.contains(&tile.id);
        layers.push(TileCanvasLayer { id: tile.id.clone(), kind: if selected_flag { "tile-selected" } else { "tile" }.into(), name: tile.name.clone(), x, y, width, height, data_url: None });
    }
    serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into())
}
//#endregion 🔖️CanvasLayers

//#region 🔖️Render
pub fn render(deck: &PresentSnapshot, selected: &[String]) -> UiNode {
    build_canvas_2d_scene(PRESENT_PLAY_SURFACE_MAIN, PRESENT_PLAY_APP_ID, Canvas2dScene { camera_x: 0.0, camera_y: 0.0, zoom: 1.0, layers_json: deck_to_canvas_layers(deck, selected) })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::present::testkit::{present_app, render as render_body};
    use crate::apps::present::PresentCommand;
    use semio_framework_plugin::testkit::meta;
    use serde_json::Value;

    #[test]
    fn renders_canvas_2d_scene() {
        let mut app = present_app();
        assert!(render_body(&mut app, PRESENT_PLAY_BODY_MAIN).contains("canvas-2d") || render_body(&mut app, PRESENT_PLAY_BODY_MAIN).contains("Canvas2d"));
    }

    #[test]
    fn definition_declares_the_canvas_2d_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, PRESENT_PLAY_BODY_MAIN);
        assert!(matches!(definition.surface_kind, SurfaceKind::Canvas2d));
        assert!(definition.options.measures.is_empty(), "animate present declares no live chrome measures");
    }

    #[test]
    fn source_frame_renders_as_actual_image_layer_behind_tiles() {
        let mut app = present_app();
        app.dispatch_typed(PresentCommand::SeedGrid(crate::apps::present::commands::grid::seed_grid::SeedGrid { rows: 1, columns: 2 }), &meta("local")).expect("seed grid");
        let deck = app.snapshot().expect("projection");
        let layers_json = deck_to_canvas_layers(&deck, &[]);
        let layers: Vec<Value> = serde_json::from_str(&layers_json).unwrap();
        assert!(!deck.source.src.trim().is_empty());
        let source_layer = layers.first().expect("source layer is first (renders behind tiles)");
        assert_eq!(source_layer.get("id").and_then(|v| v.as_str()), Some("source-frame"));
        assert_eq!(source_layer.get("kind").and_then(|v| v.as_str()), Some("image"));
        assert_eq!(source_layer.get("dataUrl").and_then(|v| v.as_str()), Some(deck.source.src.as_str()));
        for tile_layer in &layers[1..] {
            assert_ne!(tile_layer.get("kind").and_then(|v| v.as_str()), Some("image"));
            assert!(tile_layer.get("dataUrl").is_none() || tile_layer.get("dataUrl") == Some(&Value::Null));
        }
    }

    #[test]
    fn deck_to_canvas_layers_omits_data_url_when_source_has_no_image() {
        let mut deck = crate::artifacts::present::default_present_snapshot();
        deck.source.src = String::new();
        let layers_json = deck_to_canvas_layers(&deck, &[]);
        let layers: Vec<Value> = serde_json::from_str(&layers_json).unwrap();
        let source_layer = layers.first().expect("source layer present");
        assert_eq!(source_layer.get("kind").and_then(|v| v.as_str()), Some("source"));
        assert!(source_layer.get("dataUrl").is_none() || source_layer.get("dataUrl") == Some(&Value::Null));
    }

    #[test]
    fn deck_to_canvas_layers_treats_pdf_kind_as_non_image() {
        let mut deck = crate::artifacts::present::default_present_snapshot();
        deck.source.kind = "pdf".into();
        let layers_json = deck_to_canvas_layers(&deck, &[]);
        let layers: Vec<Value> = serde_json::from_str(&layers_json).unwrap();
        let source_layer = layers.first().expect("source layer present");
        assert_eq!(source_layer.get("kind").and_then(|v| v.as_str()), Some("source"));
    }
}
//#endregion 🧪️Tests
