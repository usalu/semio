//! 🖼️ Layout play app — shared canvas-scene chrome. Config-typed helpers with more than one taxonomy
//! consumer (both windows' `render()` call `canvas_layers`; `active_page` is also reached by the pointer
//! commands' hit-testing) live here rather than in the artifact engine, per the rule: a helper that takes
//! an exact window-state type (`LayoutWindowConfig`) as a parameter stays at this shared renderer no matter how many
//! consumers it has, because artifacts must never depend on apps.

use crate::editor::layout::modes::edit::windows::blueprint::config::LayoutWindowConfig;
use crate::editor::layout::modes::edit::windows::blueprint::{select, transform};
use crate::editor::layout::modes::edit::windows::blueprint::transient::LayoutWindowTransient;
use crate::editor::layout::LayoutInteractionSnapshot;
use crate::editor::layout::engine::scene::build_interactive_display_list;
use crate::{LayoutSnapshot, Page};
use serde_json::{json, Value};

//#region 🔖️ActivePage
/// 👁️ The page shown/edited on the Blueprint surface — the config's `active_page_id`, falling back to
/// the document's first page when that id no longer resolves.
pub fn active_page<'a>(doc: &'a LayoutSnapshot, config: &LayoutWindowConfig) -> Option<&'a Page> {
    doc.pages.iter().find(|page| page.id == config.active_page_id).or_else(|| doc.pages.first())
}
//#endregion 🔖️ActivePage

//#region 🔖️CanvasScene
fn rect_segments(x: f64, y: f64, width: f64, height: f64) -> Value {
    rotated_rect_segments(x, y, width, height, 0.0)
}

fn rotated_rect_segments(x: f64, y: f64, width: f64, height: f64, rotation: f64) -> Value {
    if rotation.abs() < 1.0e-6 {
        return json!([
            { "kind": "move", "to": [x, y] },
            { "kind": "line", "to": [x + width, y] },
            { "kind": "line", "to": [x + width, y + height] },
            { "kind": "line", "to": [x, y + height] },
            { "kind": "close" },
        ]);
    }
    let cx = x + width * 0.5;
    let cy = y + height * 0.5;
    let (sin, cos) = rotation.sin_cos();
    let corners = [(x, y), (x + width, y), (x + width, y + height), (x, y + height)];
    let mapped: Vec<[f64; 2]> = corners.iter().map(|(px, py)| {
        let dx = px - cx;
        let dy = py - cy;
        [cx + dx * cos - dy * sin, cy + dx * sin + dy * cos]
    }).collect();
    json!([
        { "kind": "move", "to": mapped[0] },
        { "kind": "line", "to": mapped[1] },
        { "kind": "line", "to": mapped[2] },
        { "kind": "line", "to": mapped[3] },
        { "kind": "close" },
    ])
}

fn rotate_mark_points(mark: &mut Value, cx: f64, cy: f64, rotation: f64) {
    let (sin, cos) = rotation.sin_cos();
    let Some(steps) = mark.as_array_mut() else { return };
    for step in steps {
        let Some(to) = step.get_mut("to").and_then(Value::as_array_mut) else { continue };
        if to.len() != 2 { continue }
        let x = to[0].as_f64().unwrap_or(0.0);
        let y = to[1].as_f64().unwrap_or(0.0);
        let dx = x - cx;
        let dy = y - cy;
        to[0] = json!(cx + dx * cos - dy * sin);
        to[1] = json!(cy + dx * sin + dy * cos);
    }
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

fn display_list_to_host_layers(list: &crate::editor::layout::engine::scene::DisplayList, blueprint: bool, drop_preview: &crate::LayoutDropPreviewState) -> Vec<Value> {
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
        let segments = rotated_rect_segments(rect.x as f64, rect.y as f64, rect.width as f64, rect.height as f64, rect.rotation as f64);
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
        let rotation = image.rotation as f64;
        let upright = rotation.abs() < 1.0e-6;
        if upright {
            if let Some(data_url) = &image.proxy_data_url {
                layers.push(json!({
                    "id": format!("{}.image", image.object_id),
                    "kind": "image",
                    "x": image.x,
                    "y": image.y,
                    "width": image.width,
                    "height": image.height,
                    "dataUrl": data_url,
                }));
                continue;
            }
        }
        let color = if image.placeholder { [0.92, 0.88, 0.84, 1.0] } else { [0.85, 0.85, 0.85, 1.0] };
        let segments = rotated_rect_segments(image.x as f64, image.y as f64, image.width as f64, image.height as f64, rotation);
        let stroke = image.placeholder.then_some(([0.75, 0.35, 0.2, 1.0], 1.0, None));
        layers.push(host_layer(format!("{}.image", image.object_id), &segments, Some(color), stroke));
        if !image.preview.is_empty() {
            let x = image.x as f64;
            let y = image.y as f64;
            let w = image.width as f64;
            let h = image.height as f64;
            let cx = x + w * 0.5;
            let cy = y + h * 0.5;
            let mut mark = match image.preview.as_str() {
                "page" => json!([{ "kind": "move", "to": [x + 4.0, y + h * 0.35] }, { "kind": "line", "to": [x + w - 4.0, y + h * 0.35] }, { "kind": "move", "to": [x + 4.0, y + h * 0.6] }, { "kind": "line", "to": [x + w - 4.0, y + h * 0.6] }]),
                "stroke" => line_segments(x + 4.0, y + h - 4.0, x + w - 4.0, y + 4.0),
                "map" => json!([{ "kind": "move", "to": [x + w * 0.5, y + 4.0] }, { "kind": "line", "to": [x + w * 0.5, y + h - 4.0] }, { "kind": "move", "to": [x + 4.0, y + h * 0.5] }, { "kind": "line", "to": [x + w - 4.0, y + h * 0.5] }]),
                "curve" => json!([{ "kind": "move", "to": [x + 4.0, y + h - 4.0] }, { "kind": "line", "to": [x + w * 0.5, y + 4.0] }, { "kind": "line", "to": [x + w - 4.0, y + h * 0.6] }]),
                _ => json!([{ "kind": "move", "to": [x + 4.0, y + 4.0] }, { "kind": "line", "to": [x + w - 4.0, y + h - 4.0] }, { "kind": "move", "to": [x + w - 4.0, y + 4.0] }, { "kind": "line", "to": [x + 4.0, y + h - 4.0] }]),
            };
            if !upright {
                rotate_mark_points(&mut mark, cx, cy, rotation);
            }
            layers.push(host_layer(format!("{}.preview", image.object_id), &mark, None, Some(([0.25, 0.25, 0.28, 0.9], 1.0, None))));
        }
    }

    for run in &list.text_runs {
        if run.content.is_empty() {
            continue;
        }
        layers.push(json!({
            "id": format!("{}.text", run.object_id),
            "kind": "text",
            "x": run.origin_x,
            "y": run.origin_y,
            "width": run.font_size * run.content.chars().count() as f32 * 0.6,
            "height": run.font_size,
            "text": { "content": run.content, "size": run.font_size },
            "fill": { "color": [0.0, 0.0, 0.0, 1.0] },
        }));
    }

    if blueprint && !drop_preview.kind.is_empty() && drop_preview.kind != "page" {
        let segments = rect_segments(drop_preview.x, drop_preview.y, LAYOUT_DROP_PREVIEW_WIDTH, LAYOUT_DROP_PREVIEW_HEIGHT);
        let fill = drop_preview_fill(&drop_preview.kind);
        layers.push(host_layer("layout.drop-preview", &segments, Some(fill), Some(([0.1, 0.45, 0.95, 0.85], 2.0, None))));
    }

    layers
}

fn gumball_layers(doc: &LayoutSnapshot, config: &LayoutWindowConfig, interaction: &LayoutInteractionSnapshot, blueprint: bool) -> Vec<Value> {
    if !blueprint {
        return Vec::new();
    }
    let utility = if config.active_utility.is_empty() { select::UTILITY_ID } else { config.active_utility.as_str() };
    let mut layers = vec![json!({ "id": "meta:utility", "role": "meta", "utility": utility })];
    if utility != transform::UTILITY_ID {
        return layers;
    }
    let options = transform::options();
    let Some(page) = active_page(doc, config) else { return layers };
    let Some(id) = interaction.ids.first() else { return layers };
    let bounds = page.frames.iter().find(|frame| frame.id() == id).map(|frame| frame.bounds()).or_else(|| {
        let parent = doc.parent_pages.iter().find(|parent| Some(&parent.id) == page.parent_page_id.as_ref())?;
        parent.frames.iter().find(|frame| frame.id() == id).map(|frame| frame.bounds())
    });
    let Some(bounds) = bounds else { return layers };
    let pivot = [bounds.x + bounds.width * 0.5, bounds.y + bounds.height * 0.5];
    layers.push(json!({
        "id": "meta:gumball",
        "role": "meta",
        "gumball": {
            "active": true,
            "space": "world",
            "pivotLayer": pivot,
            "pivotModel": pivot,
            "selectionIds": interaction.ids,
            "config": { "moveAxes": options.move_axes, "rotate": options.rotate, "scaleAxes": options.scale_axes, "scaleUniform": options.scale_uniform }
        }
    }));
    layers
}

/// 🖼️ Builds the host canvas-2d layer JSON for the given surface (`blueprint` or `preview`) — the
/// single shared render path both `🎭️modes/✏️edit/🪟️windows/📐️blueprint` and `…/👁️preview` call.
pub fn canvas_layers(doc: &LayoutSnapshot, config: &LayoutWindowConfig, transient: &LayoutWindowTransient, interaction: &LayoutInteractionSnapshot, blueprint: bool) -> String {
    let page = match active_page(doc, config) {
        Some(page) => page,
        None => return "[]".into(),
    };
    let camera = &config.camera;
    let list = build_interactive_display_list(doc, page, &page.id, &interaction.ids, interaction.hovered_id(), blueprint, camera.x, camera.y, camera.zoom);
    let mut layers = display_list_to_host_layers(&list, blueprint, &transient.drop_preview);
    layers.extend(gumball_layers(doc, config, interaction, blueprint));
    serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into())
}
//#endregion 🔖️CanvasScene

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
