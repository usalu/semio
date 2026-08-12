//! 🖼️ Layout play app — shared canvas-scene chrome. Config-typed helpers with more than one taxonomy
//! consumer (both windows' `render()` call `canvas_layers`; `active_page` is also reached by the pointer
//! commands' hit-testing) live here rather than in the artifact engine, per the rule: a helper that takes
//! an app-only view-state type (`LayoutConfig`) as a parameter stays at app level no matter how many
//! consumers it has, because artifacts must never depend on apps.

use crate::apps::layout::config::LayoutConfig;
use crate::apps::layout::engine::scene::{LayoutEngine, build_display_list_for_page};
use crate::artifacts::layout::{LayoutSnapshot, Page};
use serde_json::{json, Value};

//#region 🔖️ActivePage
/// 👁️ The page shown/edited on the Blueprint surface — the config's `active_page_id`, falling back to
/// the document's first page when that id no longer resolves.
pub fn active_page<'a>(doc: &'a LayoutSnapshot, config: &LayoutConfig) -> Option<&'a Page> {
    doc.pages.iter().find(|page| page.id == config.active_page_id).or_else(|| doc.pages.first())
}
//#endregion 🔖️ActivePage

//#region 🔖️CanvasScene
fn rect_segments(x: f64, y: f64, width: f64, height: f64) -> Value {
    json!([
        { "kind": "move", "to": [x, y] },
        { "kind": "line", "to": [x + width, y] },
        { "kind": "line", "to": [x + width, y + height] },
        { "kind": "line", "to": [x, y + height] },
        { "kind": "close" },
    ])
}

fn line_segments(x0: f64, y0: f64, x1: f64, y1: f64) -> Value {
    json!([
        { "kind": "move", "to": [x0, y0] },
        { "kind": "line", "to": [x1, y1] },
    ])
}

fn host_layer(id: impl Into<String>, segments: &Value, fill: Option<[f32; 4]>, stroke: Option<([f32; 4], f64, Option<[f64; 2]>)>) -> Value {
    let mut layer = json!({ "id": id.into(), "segments": segments });
    if let Some(color) = fill {
        layer["fill"] = json!({ "color": color });
    }
    if let Some((color, width, dash)) = stroke {
        let mut stroke_value = json!({ "color": color, "width": width });
        if let Some(dash) = dash {
            stroke_value["dash"] = json!(dash);
        }
        layer["stroke"] = stroke_value;
    }
    layer
}

fn guide_stroke_color(kind: &str) -> [f32; 4] {
    match kind {
        "margin" => [0.75, 0.2, 0.2, 0.35],
        "column" => [0.2, 0.45, 0.85, 0.25],
        "baseline" => [0.5, 0.5, 0.5, 0.2],
        _ => [0.3, 0.3, 0.3, 0.3],
    }
}

fn drop_preview_fill(kind: &str) -> [f32; 4] {
    match kind {
        "rect" => [0.85, 0.88, 0.92, 0.45],
        "text" => [0.2, 0.55, 0.9, 0.25],
        "image" => [0.85, 0.45, 0.2, 0.25],
        _ => [0.5, 0.5, 0.5, 0.3],
    }
}

const LAYOUT_DROP_PREVIEW_WIDTH: f64 = 200.0;
const LAYOUT_DROP_PREVIEW_HEIGHT: f64 = 120.0;

fn display_list_to_host_layers(list: &crate::apps::layout::engine::scene::DisplayList, blueprint: bool, drop_preview: &crate::artifacts::layout::LayoutDropPreviewState) -> Vec<Value> {
    let mut layers = Vec::new();

    let page_bg = if blueprint { [0.97, 0.97, 0.98, 1.0] } else { [1.0, 1.0, 1.0, 1.0] };
    layers.push(host_layer("layout.page-bg", &rect_segments(0.0, 0.0, list.page_width as f64, list.page_height as f64), Some(page_bg), None));

    if blueprint {
        for guide in &list.guides {
            let color = guide_stroke_color(&guide.kind);
            let segments = if guide.rect.height <= 0.0 { line_segments(guide.rect.x, guide.rect.y, guide.rect.x + guide.rect.width, guide.rect.y) } else { rect_segments(guide.rect.x, guide.rect.y, guide.rect.width, guide.rect.height) };
            layers.push(host_layer(format!("layout.guide.{}", guide.kind), &segments, None, Some((color, 1.0, None))));
        }
    }

    for rect in &list.rects {
        let segments = rect_segments(rect.x as f64, rect.y as f64, rect.width as f64, rect.height as f64);
        let fill = rect.fill.as_ref().map(|color| color.0);
        let dash = (blueprint && rect.inherited).then_some([4.0, 3.0]);
        let stroke = if let Some(stroke_color) = &rect.stroke {
            let width = if rect.selected {
                2.5
            } else if rect.hovered {
                1.75
            } else {
                1.0
            };
            Some((stroke_color.0, width, dash))
        } else if rect.selected && blueprint {
            Some(([0.1, 0.45, 0.95, 1.0], 2.0, None))
        } else if rect.hovered && blueprint {
            Some(([0.95, 0.72, 0.15, 1.0], 1.5, None))
        } else {
            None
        };
        layers.push(host_layer(rect.object_id.clone(), &segments, fill, stroke));
    }

    for image in &list.images {
        let color = if image.placeholder { [0.92, 0.88, 0.84, 1.0] } else { [0.85, 0.85, 0.85, 1.0] };
        let segments = rect_segments(image.x as f64, image.y as f64, image.width as f64, image.height as f64);
        let stroke = image.placeholder.then_some(([0.75, 0.35, 0.2, 1.0], 1.0, None));
        layers.push(host_layer(format!("{}.image", image.object_id), &segments, Some(color), stroke));
    }

    for run in &list.text_runs {
        if run.glyphs.is_empty() {
            continue;
        }
        let mut segments = Vec::new();
        for glyph in &run.glyphs {
            let scale = (glyph.font_size / 16.0) as f64;
            let width = 0.45 * scale;
            let height = glyph.font_size as f64 * scale;
            let x = glyph.x as f64;
            let y = glyph.y as f64;
            segments.push(json!({ "kind": "move", "to": [x, y - height] }));
            segments.push(json!({ "kind": "line", "to": [x + width, y - height] }));
            segments.push(json!({ "kind": "line", "to": [x + width, y] }));
            segments.push(json!({ "kind": "line", "to": [x, y] }));
            segments.push(json!({ "kind": "close" }));
        }
        layers.push(host_layer(format!("{}.glyphs", run.object_id), &json!(segments), Some([0.0, 0.0, 0.0, 1.0]), None));
    }

    if blueprint && !drop_preview.kind.is_empty() && drop_preview.kind != "page" {
        let segments = rect_segments(drop_preview.x, drop_preview.y, LAYOUT_DROP_PREVIEW_WIDTH, LAYOUT_DROP_PREVIEW_HEIGHT);
        let fill = drop_preview_fill(&drop_preview.kind);
        layers.push(host_layer("layout.drop-preview", &segments, Some(fill), Some(([0.1, 0.45, 0.95, 0.85], 2.0, None))));
    }

    layers
}

/// 🖼️ Builds the host canvas-2d layer JSON for the given surface (`blueprint` or `preview`) — the
/// single shared render path both `🎭️modes/✏️edit/🪟️windows/📐️blueprint` and `…/👁️preview` call.
pub fn canvas_layers(engine: &mut LayoutEngine, doc: &LayoutSnapshot, config: &LayoutConfig, blueprint: bool) -> String {
    let page = match active_page(doc, config) {
        Some(page) => page,
        None => return "[]".into(),
    };
    let list = build_display_list_for_page(engine, doc, page, &page.id, &config.selected_ids, config.hovered_id.as_deref(), blueprint);
    let layers = display_list_to_host_layers(&list, blueprint, &config.drop_preview);
    serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into())
}
//#endregion 🔖️CanvasScene

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn active_page_falls_back_to_first_page_when_config_id_unresolved() {
        let doc = crate::artifacts::layout::schema::default_document();
        let config = LayoutConfig { active_page_id: "no-such-page".into(), ..LayoutConfig::default() };
        let page = active_page(&doc, &config).expect("falls back to first page");
        assert_eq!(page.id, doc.pages[0].id);
    }

    #[test]
    fn canvas_layers_renders_the_page_background() {
        let doc = crate::artifacts::layout::schema::default_document();
        let config = LayoutConfig::default();
        let mut engine = LayoutEngine::new();
        let json = canvas_layers(&mut engine, &doc, &config, true);
        assert!(json.contains("layout.page-bg"));
    }
}
//#endregion 🧪️Tests
