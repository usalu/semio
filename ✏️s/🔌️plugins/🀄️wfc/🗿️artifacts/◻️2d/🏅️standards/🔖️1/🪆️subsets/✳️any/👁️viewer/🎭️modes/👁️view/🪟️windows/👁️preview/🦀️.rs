//! 👁️ WFC 2D viewer — the `wfc-2d-board` window: the authored board on `SurfaceKind::Canvas2d`.
//!
//! A viewer never subscribes to the solve job, so it paints each slot with its PINNED tile only —
//! the honest read-only view of what the document itself says. Unpinned slots render as outlines.
//! The layer projection is duplicated from the editor's preview on purpose: a viewer file importing
//! through the sibling editor module is exactly what `policyViewerPurityBreaches` refuses.

use crate::schema::snapshot::{Wfc2dColor, Wfc2dPathSegment, Wfc2dSlot, Wfc2dTile, Wfc2dTileMedia};
use crate::Wfc2dSnapshot;
use semio_framework_plugin::{scene_surface, BuiltNode, Canvas2dScene, LocalizedLabel, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const WFC_2D_VIEW_WINDOW: &str = "wfc-2d-board";
pub const WFC_2D_VIEW_BODY: &str = "wfc.wfc2d.board";
const WFC_2D_VIEW_SURFACE: &str = "wfc.wfc2d.board";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WFC_2D_VIEW_WINDOW.into(),
        label: LocalizedLabel::native("Board", "Tafel"),
        body_key: WFC_2D_VIEW_BODY.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "preview".into(),
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


//#region 🔭️Fit
/// 🔭️ How wide the whole board is drawn in canvas world units. Slot coordinates are authored in
/// DOMAIN units (a corridor two units wide), which at camera zoom 1 would be two PIXELS — the board
/// rendered as an unreadable speck at the pane centre. The projection below scales the document's own
/// bounds onto this box around the world origin, so every example fills the pane at the default
/// camera whatever units it was authored in.
const WFC_2D_PREVIEW_EXTENT: f64 = 640.0;

/// 🔭️ The similarity transform `WFC_2D_PREVIEW_EXTENT` implies for one document — uniform, so tile
/// media never stretches, and centred, so the board sits under the default camera.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Wfc2dPreviewFit {
    pub scale: f64,
    pub offset_x: f64,
    pub offset_y: f64,
}

impl Wfc2dPreviewFit {
    pub fn x(&self, value: f64) -> f64 {
        value * self.scale + self.offset_x
    }
    pub fn y(&self, value: f64) -> f64 {
        value * self.scale + self.offset_y
    }
}

/// 🔭️ The fit a document's slot rectangles imply; an empty board keeps the identity transform.
pub fn preview_fit(document: &Wfc2dSnapshot) -> Wfc2dPreviewFit {
    let identity = Wfc2dPreviewFit { scale: 1.0, offset_x: 0.0, offset_y: 0.0 };
    let Some(first) = document.slots.first() else { return identity };
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (first.x, first.y, first.x + first.width, first.y + first.height);
    for slot in &document.slots {
        min_x = min_x.min(slot.x);
        min_y = min_y.min(slot.y);
        max_x = max_x.max(slot.x + slot.width);
        max_y = max_y.max(slot.y + slot.height);
    }
    let extent = (max_x - min_x).max(max_y - min_y);
    if !extent.is_finite() || extent <= f64::EPSILON {
        return identity;
    }
    let scale = WFC_2D_PREVIEW_EXTENT / extent;
    Wfc2dPreviewFit { scale, offset_x: -(min_x + max_x) / 2.0 * scale, offset_y: -(min_y + max_y) / 2.0 * scale }
}

/// 🔭️ One slot as the canvas draws it — the same rectangle through `preview_fit`.
fn fitted_slot(slot: &Wfc2dSlot, fit: Wfc2dPreviewFit) -> Wfc2dSlot {
    Wfc2dSlot { id: slot.id.clone(), x: fit.x(slot.x), y: fit.y(slot.y), width: slot.width * fit.scale, height: slot.height * fit.scale, pinned_tile_id: slot.pinned_tile_id.clone() }
}
//#endregion 🔭️Fit

//#region 🔖️Paint
fn rgba(color: &Wfc2dColor) -> String {
    format!("[{:.4},{:.4},{:.4},{:.4}]", f64::from(color.r) / 255.0, f64::from(color.g) / 255.0, f64::from(color.b) / 255.0, f64::from(color.a) / 255.0)
}

fn point(value: [f64; 2]) -> String {
    format!("[{:.6},{:.6}]", value[0], value[1])
}

fn segment_json(segment: &Wfc2dPathSegment) -> String {
    match segment {
        Wfc2dPathSegment::Close => "{\"kind\":\"close\"}".to_string(),
        Wfc2dPathSegment::Move { to } => format!("{{\"kind\":\"move\",\"to\":{}}}", point(*to)),
        Wfc2dPathSegment::Line { to } => format!("{{\"kind\":\"line\",\"to\":{}}}", point(*to)),
        Wfc2dPathSegment::Quad { ctrl, to } => format!("{{\"kind\":\"quad\",\"ctrl\":{},\"to\":{}}}", point(*ctrl), point(*to)),
        Wfc2dPathSegment::Cubic { ctrl1, ctrl2, to } => format!("{{\"kind\":\"cubic\",\"ctrl1\":{},\"ctrl2\":{},\"to\":{}}}", point(*ctrl1), point(*ctrl2), point(*to)),
    }
}

fn slot_layers(slot: &Wfc2dSlot, tile: Option<&Wfc2dTile>, layers: &mut Vec<String>) {
    let name = tile.map_or_else(|| slot.id.clone(), |tile| format!("{}·{}", slot.id, tile.id));
    layers.push(format!(
        "{{\"id\":\"slot-{}\",\"kind\":\"rect\",\"name\":{},\"x\":{:.6},\"y\":{:.6},\"width\":{:.6},\"height\":{:.6}}}",
        slot.id,
        protocol::json::to_json_string(&name),
        slot.x,
        slot.y,
        slot.width,
        slot.height
    ));
    let Some(tile) = tile else { return };
    if let Some(data_url) = crate::standards::v1::subsets::any::io::snapshot::binary::tile_media_png_data_url(&tile.media) {
        layers.push(format!(
            "{{\"id\":\"tile-{}-{}-bitmap\",\"kind\":\"image\",\"dataUrl\":{},\"x\":{:.6},\"y\":{:.6},\"width\":{:.6},\"height\":{:.6}}}",
            slot.id,
            tile.id,
            protocol::json::to_json_string(&data_url),
            slot.x,
            slot.y,
            slot.width,
            slot.height
        ));
    }
    if let Wfc2dTileMedia::Vector { paths } = &tile.media {
        for (index, path) in paths.iter().enumerate() {
            let segments: Vec<String> = path.segments.iter().map(segment_json).collect();
            let fill = path.fill.as_ref().map_or_else(|| "null".to_string(), |color| format!("{{\"color\":{}}}", rgba(color)));
            let stroke = path.stroke.as_ref().map_or_else(|| "null".to_string(), |color| format!("{{\"color\":{},\"width\":{:.6}}}", rgba(color), path.stroke_width));
            layers.push(format!(
                "{{\"id\":\"tile-{}-{}-{index}\",\"kind\":\"path\",\"transform\":[{:.6},0.0,0.0,{:.6},{:.6},{:.6}],\"segments\":[{}],\"fill\":{fill},\"stroke\":{stroke}}}",
                slot.id,
                tile.id,
                slot.width,
                slot.height,
                slot.x,
                slot.y,
                segments.join(",")
            ));
        }
    }
}

/// 🖼️ The authored board's layer array — one entry per slot, in document order.
pub fn board_layers_json(document: &Wfc2dSnapshot) -> String {
    let fit = preview_fit(document);
    let mut layers: Vec<String> = Vec::new();
    for slot in &document.slots {
        let tile = slot.pinned_tile_id.as_ref().and_then(|id| document.tiles.iter().find(|tile| &tile.id == id));
        slot_layers(&fitted_slot(slot, fit), tile, &mut layers);
    }
    format!("[{}]", layers.join(","))
}
//#endregion 🔖️Paint

//#region 🔖️Render
pub fn render(document: &Wfc2dSnapshot) -> UiAssemblyResult<BuiltNode> {
    scene_surface(WFC_2D_VIEW_SURFACE, semio_framework_ui_contract::SurfaceKind::Canvas2d, &Canvas2dScene::base(0.0, 0.0, 1.0, board_layers_json(document)))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
