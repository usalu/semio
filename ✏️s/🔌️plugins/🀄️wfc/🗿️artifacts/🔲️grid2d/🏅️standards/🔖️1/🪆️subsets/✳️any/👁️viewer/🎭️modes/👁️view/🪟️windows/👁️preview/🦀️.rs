//! 👁️ 2D-grid viewer — `preview` window: a read-only `Canvas2d` render of the authored grid. Every
//! cell draws its outline; a PINNED cell additionally draws its tile's media scaled into the cell
//! rect, and a MASKED cell draws nothing at all. Independent render from the sibling
//! mutation-capable surface — this file must not import through `crate::editor`
//! (`policyViewerPurityBreaches`).

use crate::schema::snapshot::{decode_palette_indices, Grid2dSnapshot, WfcColor, WfcPathSegment, WfcTile2d, WfcTileMedia2d};
use semio_framework_plugin::{BuiltNode, Canvas2dScene, LocalizedLabel, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "wfc-grid2d-view-preview";
pub const BODY_KEY: &str = "wfc.grid2d.view.preview";
pub const SURFACE_ID: &str = "wfc.grid2d.surface.view-preview";
const MAX_BITMAP_PIXEL_RECTS: usize = 64;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::grid2d::create_grid2d_viewer`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Preview", "Vorschau"),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "preview".into(),
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

//#region 🔖️Render
fn color_rgba(color: WfcColor) -> Value {
    json!([f64::from(color.r) / 255.0, f64::from(color.g) / 255.0, f64::from(color.b) / 255.0, f64::from(color.a) / 255.0])
}

fn color_hex(color: WfcColor) -> String {
    format!("#{:02x}{:02x}{:02x}", color.r.min(255), color.g.min(255), color.b.min(255))
}

fn tile_layers(tile: &WfcTile2d, x: u32, y: u32, cell_width: f64, cell_height: f64) -> Vec<Value> {
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
                    "id": format!("pin-{x}-{y}-path-{index}"),
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
            if cells == 0 || palette.is_empty() || cells > MAX_BITMAP_PIXEL_RECTS {
                return vec![json!({ "id": format!("pin-{x}-{y}"), "name": tile.id, "x": origin_x, "y": origin_y, "width": cell_width, "height": cell_height, "color": "#64748b" })];
            }
            let pixel_width = cell_width / f64::from(*width);
            let pixel_height = cell_height / f64::from(*height);
            (0..cells)
                .map(|offset| {
                    let color = palette[usize::from(*indices.get(offset).unwrap_or(&0)) % palette.len()];
                    json!({
                        "id": format!("pin-{x}-{y}-px-{offset}"),
                        "x": origin_x + (offset % (*width as usize)) as f64 * pixel_width,
                        "y": origin_y + (offset / (*width as usize)) as f64 * pixel_height,
                        "width": pixel_width,
                        "height": pixel_height,
                        "color": color_hex(color),
                    })
                })
                .collect()
        }
        WfcTileMedia2d::Image { child } => {
            vec![json!({ "id": format!("pin-{x}-{y}"), "name": child.child_id, "x": origin_x, "y": origin_y, "width": cell_width, "height": cell_height, "color": "#0f766e" })]
        }
    }
}

/// 🧱️ The canvas layer list: one outline per unmasked cell, plus every pinned cell's tile media.
pub fn layers_json(document: &Grid2dSnapshot) -> String {
    let mut layers: Vec<Value> = Vec::new();
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
    for cell in &document.pinned {
        if let Some(tile) = document.tiles.iter().find(|tile| tile.id == cell.tile_id) {
            layers.extend(tile_layers(tile, cell.x, cell.y, document.cell_width, document.cell_height));
        }
    }
    Value::Array(layers).to_string()
}

pub fn scene(document: &Grid2dSnapshot) -> Canvas2dScene {
    let centre_x = f64::from(document.width) * document.cell_width * 0.5;
    let centre_y = f64::from(document.height) * document.cell_height * 0.5;
    Canvas2dScene::base(centre_x, centre_y, 1.0, layers_json(document))
}

pub fn render(document: &Grid2dSnapshot) -> UiAssemblyResult<BuiltNode> {
    semio_framework_plugin::scene_surface(SURFACE_ID, semio_framework_ui_contract::SurfaceKind::Canvas2d, &scene(document))
}
//#endregion 🔖️Render
