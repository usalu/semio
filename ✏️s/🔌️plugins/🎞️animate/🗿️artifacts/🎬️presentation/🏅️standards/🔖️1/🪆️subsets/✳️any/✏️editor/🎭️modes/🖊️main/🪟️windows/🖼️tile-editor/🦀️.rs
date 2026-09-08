//! 🖼️ Animate presentation app — the tile-editor window: the canvas 2d surface rendering the source figure
//! backdrop plus its crop tiles.

use crate::{FigureTileFrame, PresentationSnapshot};
use semio_framework_plugin::{LocalizedLabel, SurfaceKind, WindowKindDefinition, WindowOptions};
use semio_framework_ui_contract::BuiltNode;
use semio_framework_ui_scene::Canvas2dScene;

//#region 🔖️Constants
pub const PRESENTATION_PLAY_WINDOW_MAIN: &str = "tile-editor";
pub const PRESENTATION_PLAY_BODY_MAIN: &str = "animate.presentation.play.main";
const PRESENTATION_PLAY_SURFACE_MAIN: &str = "animate.presentation.play";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::animate::create_animate_presentation_app`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: PRESENTATION_PLAY_WINDOW_MAIN.into(),
        label: LocalizedLabel::native("Tile editor", "Kacheleditor"),
        body_key: PRESENTATION_PLAY_BODY_MAIN.into(),
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
#[derive(value_derive::ToValue)]
#[value(rename_all = "camelCase")]
struct TileCanvasLayer {
    id: String,
    kind: String,
    name: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    /// 🖼️ Image src for `kind: "image"` layers, rendered by both the React and wgpu canvas-2d hosts.
    #[value(skip_serializing_if = "Option::is_none")]
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
fn deck_to_canvas_layers(deck: &PresentationSnapshot) -> String {
    const SCALE: f64 = 1000.0;
    let (source, tiles) = crate::presentation_working_scene(deck);
    let mut layers = Vec::new();
    let (sx, sy, sw, sh) = frame_to_canvas(&source.frame, SCALE);
    let has_image_src = !source.src.trim().is_empty() && source.kind != "pdf";
    layers.push(TileCanvasLayer { id: "source-frame".into(), kind: if has_image_src { "image".into() } else { "source".into() }, name: source.src.clone(), x: sx, y: sy, width: sw, height: sh, data_url: has_image_src.then(|| source.src.clone()) });
    for tile in &tiles {
        let (x, y, width, height) = frame_to_canvas(&tile.crop, SCALE);
        layers.push(TileCanvasLayer { id: tile.id.clone(), kind: "tile".into(), name: tile.name.clone(), x, y, width, height, data_url: None });
    }
    dsl::os_pack::json::to_json_string(&layers)
}
//#endregion 🔖️CanvasLayers

//#region 🔖️Render
pub fn render(deck: &PresentationSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let scene = Canvas2dScene { camera_x: 0.0, camera_y: 0.0, zoom: 1.0, layers_json: deck_to_canvas_layers(deck), snapshot: None };
    semio_framework_plugin::scene_surface(PRESENTATION_PLAY_SURFACE_MAIN, semio_framework_ui_contract::SurfaceKind::Canvas2d, &scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
