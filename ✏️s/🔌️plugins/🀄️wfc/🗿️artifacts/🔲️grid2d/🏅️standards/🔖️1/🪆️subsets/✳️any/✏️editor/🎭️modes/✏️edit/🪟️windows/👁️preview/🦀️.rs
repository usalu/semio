//! 👁️ 2D-grid editor — the `preview` window: a read-only `Canvas2d` pane that draws each SOLVED
//! cell's tile media scaled into that cell's rect.
//!
//! The assignment it draws comes from the `s.wfc.grid2d.solve` INFERENCE, cached as JSON in this
//! pane's own window config by the `solve` command — never from the document, which persists only
//! the problem. Until `solve` has run the pane draws the bare cell outlines, which is the honest
//! "nothing inferred yet" state rather than a spinner that never ends.

use crate::editor::grid2d::window::Grid2dWindowConfig;
use crate::schema::inferences::Grid2dInferenceCommit;
use crate::schema::snapshot::{decode_palette_indices, Grid2dSnapshot, WfcColor, WfcPathSegment, WfcTile2d, WfcTileMedia2d};
use semio_framework_plugin::{ActionDefinition, ActionKind, BuiltNode, Canvas2dScene, LocalizedLabel, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "wfc-grid2d-preview";
pub const BODY_KEY: &str = "wfc.grid2d.preview";
pub const SURFACE_ID: &str = "wfc.grid2d.surface.preview";

/// 🎨️ How many bitmap pixels one tile may draw as individual rects before the pane falls back to a
/// single averaged swatch — a 12×12 board of 8×8 tiles is already 9 216 rects, and the surface doc
/// is fixed-capacity.
const MAX_BITMAP_PIXEL_RECTS: usize = 64;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::grid2d::create_grid2d_editor`.
pub fn definition() -> WindowKindDefinition {
    let mut actions = vec![
        ActionDefinition::bounded_catalog("solve", LocalizedLabel::native("Solve", "Lösen"), ActionKind::View),
        ActionDefinition::bounded_catalog("set-camera", LocalizedLabel::native("Set Camera", "Kamera setzen"), ActionKind::View),
    ];
    for action in &mut actions {
        action.semantics.execution.interactive_job = semio_framework::InteractiveJobClassification::Migrated;
    }
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Preview", "Vorschau"),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "preview".into(),
        options: WindowOptions::default(),
        actions,
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

//#region 🔖️Render
fn color_rgba(color: WfcColor) -> Value {
    json!([f64::from(color.r) / 255.0, f64::from(color.g) / 255.0, f64::from(color.b) / 255.0, f64::from(color.a) / 255.0])
}

fn color_hex(color: WfcColor) -> String {
    format!("#{:02x}{:02x}{:02x}", color.r.min(255), color.g.min(255), color.b.min(255))
}

/// 🎨️ The palette-weighted average colour of a bitmap tile — the swatch a tile too large to draw
/// pixel-by-pixel falls back to.
fn average_color(palette: &[WfcColor], indices: &[u8]) -> WfcColor {
    if palette.is_empty() {
        return WfcColor { r: 100, g: 116, b: 139, a: 255 };
    }
    let mut totals = [0u64; 4];
    let mut count = 0u64;
    for index in indices {
        let color = palette[usize::from(*index) % palette.len()];
        totals[0] += u64::from(color.r);
        totals[1] += u64::from(color.g);
        totals[2] += u64::from(color.b);
        totals[3] += u64::from(color.a);
        count += 1;
    }
    if count == 0 {
        return palette[0];
    }
    WfcColor { r: (totals[0] / count) as u32, g: (totals[1] / count) as u32, b: (totals[2] / count) as u32, a: (totals[3] / count) as u32 }
}

/// 🖼️ Every canvas layer one solved cell contributes, scaled into its own cell rect.
fn cell_layers(tile: &WfcTile2d, x: u32, y: u32, cell_width: f64, cell_height: f64) -> Vec<Value> {
    let origin_x = f64::from(x) * cell_width;
    let origin_y = f64::from(y) * cell_height;
    match &tile.media {
        WfcTileMedia2d::Vector { paths } => paths
            .iter()
            .enumerate()
            .map(|(index, path)| {
                let segments: Vec<Value> = path
                    .segments
                    .iter()
                    .map(|segment| match segment {
                        WfcPathSegment::MoveTo { to } => json!({ "kind": "move", "to": [to.x, to.y] }),
                        WfcPathSegment::LineTo { to } => json!({ "kind": "line", "to": [to.x, to.y] }),
                        WfcPathSegment::QuadTo { ctrl, to } => json!({ "kind": "quad", "ctrl": [ctrl.x, ctrl.y], "to": [to.x, to.y] }),
                        WfcPathSegment::CubicTo { ctrl1, ctrl2, to } => json!({ "kind": "cubic", "ctrl1": [ctrl1.x, ctrl1.y], "ctrl2": [ctrl2.x, ctrl2.y], "to": [to.x, to.y] }),
                        WfcPathSegment::Close => json!({ "kind": "close" }),
                    })
                    .collect();
                let mut layer = json!({
                    "id": format!("cell-{x}-{y}-path-{index}"),
                    // 🔁️ `[a, b, c, d, e, f]`: scale unit tile space into the cell rect, then translate.
                    "transform": [cell_width, 0.0, 0.0, cell_height, origin_x, origin_y],
                    "segments": segments,
                });
                if let Some(fill) = path.fill {
                    layer["fill"] = json!({ "color": color_rgba(fill) });
                }
                if let Some(stroke) = path.stroke {
                    layer["stroke"] = json!({ "color": color_rgba(stroke), "width": path.stroke_width });
                }
                layer
            })
            .collect(),
        WfcTileMedia2d::Bitmap { width, height, palette, pixels } => {
            let indices = decode_palette_indices(pixels);
            let cells = (*width as usize) * (*height as usize);
            if cells == 0 || palette.is_empty() {
                return vec![json!({ "id": format!("cell-{x}-{y}"), "name": tile.id, "x": origin_x, "y": origin_y, "width": cell_width, "height": cell_height, "color": "#64748b" })];
            }
            if cells > MAX_BITMAP_PIXEL_RECTS {
                return vec![json!({
                    "id": format!("cell-{x}-{y}"),
                    "name": tile.id,
                    "x": origin_x,
                    "y": origin_y,
                    "width": cell_width,
                    "height": cell_height,
                    "color": color_hex(average_color(palette, &indices)),
                })];
            }
            let pixel_width = cell_width / f64::from(*width);
            let pixel_height = cell_height / f64::from(*height);
            (0..cells)
                .map(|offset| {
                    let color = palette[usize::from(*indices.get(offset).unwrap_or(&0)) % palette.len()];
                    let pixel_x = (offset % (*width as usize)) as f64;
                    let pixel_y = (offset / (*width as usize)) as f64;
                    json!({
                        "id": format!("cell-{x}-{y}-px-{offset}"),
                        "x": origin_x + pixel_x * pixel_width,
                        "y": origin_y + pixel_y * pixel_height,
                        "width": pixel_width,
                        "height": pixel_height,
                        "color": color_hex(color),
                    })
                })
                .collect()
        }
        // 🧸️ A composed image child is not resolvable from inside the guest render pass, so the cell
        // draws a labelled placeholder rather than silently nothing.
        WfcTileMedia2d::Image { child } => vec![json!({
            "id": format!("cell-{x}-{y}"),
            "name": child.child_id,
            "x": origin_x,
            "y": origin_y,
            "width": cell_width,
            "height": cell_height,
            "color": "#0f766e",
        })],
    }
}

/// 🧱️ The canvas layer list — the solved assignment when one is cached, else the bare cell grid.
pub fn layers_json(document: &Grid2dSnapshot, commit: Option<&Grid2dInferenceCommit>) -> String {
    let mut layers: Vec<Value> = Vec::new();
    match commit {
        Some(commit) if !commit.contradiction && !commit.assignments.is_empty() => {
            for (x, y, tile_id) in &commit.assignments {
                match document.tiles.iter().find(|tile| &tile.id == tile_id) {
                    Some(tile) => layers.extend(cell_layers(tile, *x, *y, document.cell_width, document.cell_height)),
                    None => layers.push(json!({
                        "id": format!("cell-{x}-{y}"),
                        "name": tile_id,
                        "x": f64::from(*x) * document.cell_width,
                        "y": f64::from(*y) * document.cell_height,
                        "width": document.cell_width,
                        "height": document.cell_height,
                        "color": "#7f1d1d",
                    })),
                }
            }
        }
        _ => {
            for y in 0..document.height {
                for x in 0..document.width {
                    if document.masked.iter().any(|cell| cell.x == x && cell.y == y) {
                        continue;
                    }
                    layers.push(json!({
                        "id": format!("cell-{x}-{y}"),
                        "x": f64::from(x) * document.cell_width,
                        "y": f64::from(y) * document.cell_height,
                        "width": document.cell_width,
                        "height": document.cell_height,
                        "color": "#1e293b",
                    }));
                }
            }
        }
    }
    Value::Array(layers).to_string()
}

/// 🏁 The cached commit this pane draws, or `None` when `solve` has not run in this pane yet.
pub fn cached_commit(config: &Grid2dWindowConfig) -> Option<Grid2dInferenceCommit> {
    if config.solve_json.trim().is_empty() {
        return None;
    }
    protocol::json::from_json_str::<Grid2dInferenceCommit>(&config.solve_json).ok()
}

pub fn scene(document: &Grid2dSnapshot, config: &Grid2dWindowConfig) -> Canvas2dScene {
    let commit = cached_commit(config);
    Canvas2dScene::base(config.camera_x, config.camera_y, if config.camera_zoom > 0.0 { config.camera_zoom } else { 1.0 }, layers_json(document, commit.as_ref()))
}

pub fn render(document: &Grid2dSnapshot, config: &Grid2dWindowConfig) -> UiAssemblyResult<BuiltNode> {
    semio_framework_plugin::scene_surface(SURFACE_ID, semio_framework_ui_contract::SurfaceKind::Canvas2d, &scene(document, config))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
