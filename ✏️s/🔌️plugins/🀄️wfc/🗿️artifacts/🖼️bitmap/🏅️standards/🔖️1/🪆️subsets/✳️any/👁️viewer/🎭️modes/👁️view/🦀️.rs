//! 👁️ Bitmap viewer — the `view` mode: the same 50/50 row the editor uses, so a document reads the
//! same way whichever surface opened it.

use crate::viewer::bitmap::modes::view::windows::{input, output};
use semio_framework_plugin::create_default_layout;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::ModeDefinition;
use semio_framework_plugin::WindowLayout;

pub const WFC_BITMAP_MODE_VIEW: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::bitmap::create_bitmap_viewer`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: WFC_BITMAP_MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ A row split, 50/50.
pub fn layout() -> WindowLayout {
    create_default_layout(&[input::WFC_BITMAP_VIEW_WINDOW_INPUT.into(), output::WFC_BITMAP_VIEW_WINDOW_OUTPUT.into()], "row", Some(&[50.0, 50.0]), Some(&["Input".into(), "Output".into()]))
}
//#endregion 🔖️Definition

//#region 🧩️ArtifactItems3
pub mod artifact_items_3 {
use crate::{schema::snapshot::BitmapColor, schema::snapshot::BitmapPinnedPixel};

//#region 🎨️CanvasLayers
/// 🚚️ The byte budget one window's `layersJson` may occupy. `Canvas2dScene` keeps `layersJson` in
/// the 32 KiB fixed-capacity surface spine — it has no lane of its own, unlike a paint-2d document
/// sync — so a bitmap whose run-merged rectangles would not fit is rendered at a coarser block size
/// instead of silently failing the whole surface encode.
pub const BITMAP_LAYERS_JSON_BUDGET_BYTES: usize = 24 * 1024;

/// 🎨️ Builds one canvas-2d layer list for a palette-indexed bitmap: a background rectangle for the
/// whole extent, one filled path per horizontal run of equal indices, and one outlined marker per
/// pinned cell. Shared by both surfaces from the ARTIFACT root so the read-only viewer never has to
/// import through the editor to draw the same pixels.
///
/// An empty `indices` slice draws the extent and the pins only — the honest rendering of an output
/// nobody has collapsed yet, and the reason a window is never structurally empty.
pub fn bitmap_layers_json(prefix: &str, width: u32, height: u32, palette: &[BitmapColor], indices: &[u8], pins: &[BitmapPinnedPixel]) -> String {
    let mut block = 1u32;
    loop {
        let layers = bitmap_layers_at_block(prefix, width, height, palette, indices, pins, block);
        let text = semio_framework_pack_json::to_string(&semio_framework_pack_json::array(layers));
        if text.len() <= BITMAP_LAYERS_JSON_BUDGET_BYTES || block >= width.max(height).max(1) {
            return text;
        }
        block *= 2;
    }
}

pub fn bitmap_rect_layer(id: &str, x: f64, y: f64, w: f64, h: f64, color: [f64; 4]) -> semio_framework_pack_json::Value {
    let segments = semio_framework_pack_json::array(vec![
        semio_framework_pack_json::object([("kind".to_string(), semio_framework_pack_json::Value::from("move")), ("to".to_string(), semio_framework_pack_json::array(vec![semio_framework_pack_json::Value::from(x), semio_framework_pack_json::Value::from(y)]))]),
        semio_framework_pack_json::object([("kind".to_string(), semio_framework_pack_json::Value::from("line")), ("to".to_string(), semio_framework_pack_json::array(vec![semio_framework_pack_json::Value::from(x + w), semio_framework_pack_json::Value::from(y)]))]),
        semio_framework_pack_json::object([("kind".to_string(), semio_framework_pack_json::Value::from("line")), ("to".to_string(), semio_framework_pack_json::array(vec![semio_framework_pack_json::Value::from(x + w), semio_framework_pack_json::Value::from(y + h)]))]),
        semio_framework_pack_json::object([("kind".to_string(), semio_framework_pack_json::Value::from("line")), ("to".to_string(), semio_framework_pack_json::array(vec![semio_framework_pack_json::Value::from(x), semio_framework_pack_json::Value::from(y + h)]))]),
        semio_framework_pack_json::object([("kind".to_string(), semio_framework_pack_json::Value::from("close"))]),
    ]);
    let fill = semio_framework_pack_json::object([
        ("kind".to_string(), semio_framework_pack_json::Value::from("solid")),
        ("color".to_string(), semio_framework_pack_json::array(color.iter().map(|channel| semio_framework_pack_json::Value::from(*channel)).collect::<Vec<_>>())),
    ]);
    semio_framework_pack_json::object([("id".to_string(), semio_framework_pack_json::Value::from(id)), ("kind".to_string(), semio_framework_pack_json::Value::from("path")), ("segments".to_string(), segments), ("fill".to_string(), fill)])
}

pub fn bitmap_layers_at_block(prefix: &str, width: u32, height: u32, palette: &[BitmapColor], indices: &[u8], pins: &[BitmapPinnedPixel], block: u32) -> Vec<semio_framework_pack_json::Value> {
    let mut layers = vec![bitmap_rect_layer(&format!("{prefix}-extent"), 0.0, 0.0, f64::from(width), f64::from(height), [0.09, 0.10, 0.12, 1.0])];
    let cells = (width as usize) * (height as usize);
    if indices.len() == cells {
        for y in (0..height).step_by(block as usize) {
            let mut x = 0u32;
            while x < width {
                let index = indices[(y * width + x) as usize];
                let mut run = block.min(width - x);
                while x + run < width && indices[(y * width + x + run) as usize] == index {
                    run += block.min(width - x - run);
                }
                if let Some(color) = palette.get(usize::from(index)) {
                    layers.push(bitmap_rect_layer(&format!("{prefix}-{x}-{y}"), f64::from(x), f64::from(y), f64::from(run), f64::from(block.min(height - y)), color.to_unit_rgba()));
                }
                x += run;
            }
        }
    }
    for pin in pins {
        if pin.x < width && pin.y < height {
            let color = palette.get(pin.color as usize).map_or([1.0, 1.0, 1.0, 1.0], |color| color.to_unit_rgba());
            layers.push(bitmap_rect_layer(&format!("{prefix}-pin-{}-{}", pin.x, pin.y), f64::from(pin.x) + 0.2, f64::from(pin.y) + 0.2, 0.6, 0.6, color));
        }
    }
    layers
}
//#endregion 🎨️CanvasLayers
}
//#endregion 🧩️ArtifactItems3
