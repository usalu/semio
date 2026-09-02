//! 🖼️ Animate viewer — the tile-editor window: a read-only canvas-2d render of the source figure
//! backdrop plus its crop tiles, built from `crate::artifacts::presentation::presentation_working_scene` (the
//! same artifact-level pure accessor the editor's own tile-editor window uses) — this file itself
//! imports nothing from the sibling editor surface (`policyViewerPurityBreaches` forbids it outright).
//! No engagement bar, no selection overlay, no tile-morph prompt: a viewer has no utilities that edit
//! and emits no mutations by construction (`ViewEmit`).
//!
//! `MediaWindowKit` (contract §2.6) was considered and rejected for this window: its `MediaView`
//! shape (`duration_ms`/`position_ms`/audio-or-video `kind`, rendered as a duration/position key-value
//! summary) models transport state for a playing clip, not a 2D tile-grid layout — forcing this
//! window's real content (a background figure plus positioned crop rectangles) through that shape
//! would lose the layout entirely. A bespoke pure render function, mirroring the editor's own
//! `Canvas2dScene` approach, is the honest fit.

use crate::artifacts::presentation::{FigureTileFrame, PresentationSnapshot};
use semio_framework_plugin::{LocalizedLabel, SurfaceKind, WindowKindDefinition, WindowOptions};
use semio_framework_ui_contract::{surface, Buildable, BuiltNode, HasBase};
use semio_framework_ui_scene::{encode, Canvas2dScene};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "animate-view-tile-editor";
pub const BODY_KEY: &str = "animate.view.tile-editor";
const SURFACE_ID: &str = "animate.presentation.view";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::animate::create_animate_presentation_viewer`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Tile editor", "Kacheleditor"),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "grid-3x3".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️CanvasLayers
/// 👁️ Read-only twin of the editor's own `TileCanvasLayer` — duplicated on purpose rather than
/// imported through the sibling editor module, which `policyViewerPurityBreaches` forbids outright.
#[derive(value_derive::ToValue)]
#[value(rename_all = "camelCase")]
struct AnimateViewTileLayer {
    id: String,
    kind: String,
    name: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    #[value(skip_serializing_if = "Option::is_none")]
    data_url: Option<String>,
}

fn frame_to_canvas(frame: &FigureTileFrame, scale: f64) -> (f64, f64, f64, f64) {
    (frame.x * scale, frame.y * scale, frame.width * scale, frame.height * scale)
}

/// 👁️ Pure `PresentationSnapshot -> layers JSON` read: the same source-figure-plus-crop-tiles content the
/// editor's own canvas renders, with no selection/engagement overlay (a viewer has neither).
fn deck_to_canvas_layers(deck: &PresentationSnapshot) -> String {
    const SCALE: f64 = 1000.0;
    let (source, tiles) = crate::artifacts::presentation::presentation_working_scene(deck);
    let mut layers = Vec::new();
    let (sx, sy, sw, sh) = frame_to_canvas(&source.frame, SCALE);
    let has_image_src = !source.src.trim().is_empty() && source.kind != "pdf";
    layers.push(AnimateViewTileLayer {
        id: "source-frame".into(),
        kind: if has_image_src { "image".into() } else { "source".into() },
        name: source.src.clone(),
        x: sx,
        y: sy,
        width: sw,
        height: sh,
        data_url: has_image_src.then(|| source.src.clone()),
    });
    for tile in &tiles {
        let (x, y, width, height) = frame_to_canvas(&tile.crop, SCALE);
        layers.push(AnimateViewTileLayer { id: tile.id.clone(), kind: "tile".into(), name: tile.name.clone(), x, y, width, height, data_url: None });
    }
    let value: serde_json::Value = dsl::ToValue::to_value(&layers).into();
    value.to_string()
}
//#endregion 🔖️CanvasLayers

//#region 🔖️Render
pub fn render(deck: &PresentationSnapshot) -> BuiltNode {
    let scene = Canvas2dScene { camera_x: 0.0, camera_y: 0.0, zoom: 1.0, layers_json: deck_to_canvas_layers(deck), snapshot: None };
    surface(encode(semio_framework_ui_contract::SurfaceKind::Canvas2d, &scene)).id(SURFACE_ID).build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn renders_canvas_2d_scene() {
        let deck = crate::artifacts::presentation::default_presentation_snapshot();
        let json_str = serde_json::to_string(&render(&deck)).unwrap();
        assert!(json_str.contains("canvas-2d") || json_str.contains("Canvas2d"));
    }

    #[test]
    fn definition_declares_the_canvas_2d_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, BODY_KEY);
        assert!(matches!(definition.surface_kind, SurfaceKind::Canvas2d));
        assert!(definition.options.measures.is_empty(), "animate view declares no live chrome measures");
        assert!(definition.actions.is_empty(), "a viewer window declares no mutating actions");
    }

    #[test]
    fn source_frame_renders_as_actual_image_layer_behind_tiles() {
        let deck = crate::artifacts::presentation::default_presentation_snapshot();
        let layers_json = deck_to_canvas_layers(&deck);
        let layers: Vec<Value> = serde_json::from_str(&layers_json).unwrap();
        let (source, _) = crate::artifacts::presentation::presentation_working_scene(&deck);
        assert!(!source.src.trim().is_empty());
        let source_layer = layers.first().expect("source layer is first (renders behind tiles)");
        assert_eq!(source_layer.get("id").and_then(|v| v.as_str()), Some("source-frame"));
        assert_eq!(source_layer.get("kind").and_then(|v| v.as_str()), Some("image"));
        assert_eq!(source_layer.get("dataUrl").and_then(|v| v.as_str()), Some(source.src.as_str()));
    }

    #[test]
    fn deck_to_canvas_layers_omits_data_url_when_source_has_no_image() {
        let base = crate::artifacts::presentation::default_presentation_snapshot();
        let (mut source, tiles) = crate::artifacts::presentation::presentation_working_scene(&base);
        source.src = String::new();
        let deck = crate::artifacts::presentation::presentation_snapshot_with_tiles(&source, &tiles);
        let layers_json = deck_to_canvas_layers(&deck);
        let layers: Vec<Value> = serde_json::from_str(&layers_json).unwrap();
        let source_layer = layers.first().expect("source layer presentation");
        assert_eq!(source_layer.get("kind").and_then(|v| v.as_str()), Some("source"));
        assert!(source_layer.get("dataUrl").is_none() || source_layer.get("dataUrl") == Some(&Value::Null));
    }
}
//#endregion 🧪️Tests
