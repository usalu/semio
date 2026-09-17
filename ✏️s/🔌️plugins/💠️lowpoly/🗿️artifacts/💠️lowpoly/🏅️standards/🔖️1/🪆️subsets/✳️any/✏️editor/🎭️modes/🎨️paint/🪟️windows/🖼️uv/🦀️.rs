//! 🖼️ Lowpoly play app — the UV window: the 2D UV-canvas paint surface. Only the paint operations it
//! shares with the Model window are scoped here (no mesh-editing/transform ops).

use crate::editor::lowpoly::config::LowpolyConfig;
use crate::editor::lowpoly::engine::LowpolyDocument;
use crate::editor::lowpoly::modes::edit::windows::model::LOWPOLY_TRANSFORM_UTILITY_DEFAULT;
use crate::editor::lowpoly::terminology::LowpolyLabels;
use crate::editor::lowpoly::view::LowpolyView;
use crate::editor::lowpoly::{lowpoly_window_engagement, lowpoly_window_measures};
use crate::LOWPOLY_PAINT_TEXTURE_SIZE;
use semio_framework_plugin::{scene_surface, Canvas2dScene, PluginAssemblyError, SurfaceKind, UtilityRef, WindowEngagementSlot, WindowKindDefinition, WindowMeasure, WindowOptions};
use std::collections::HashMap;

//#region 🔖️Constants
pub const LOWPOLY_PLAY_WINDOW_UV: &str = "lowpoly-uv";
pub const LOWPOLY_PLAY_BODY_UV: &str = "lowpoly.play.uv";
const LOWPOLY_PLAY_SURFACE_UV: &str = "lowpoly.play.uv";

pub const LOWPOLY_UV_ACTIONS: &[&str] = &["addPaintLayer", "paintStrokeEnd", "paintFill", "fillBucket"];
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::lowpoly::create_lowpoly_app`.
pub fn definition() -> WindowKindDefinition {
    let projection = crate::schema::default_snapshot();
    let config = LowpolyConfig::default();
    let labels = semio_framework_plugin::resolve_labels::<LowpolyLabels>(&semio_framework_plugin::ViewModel::default());
    let engagement = lowpoly_window_engagement(LowpolyView { snapshot: &projection, config: &config }, LOWPOLY_TRANSFORM_UTILITY_DEFAULT, labels);
    WindowKindDefinition {
        id: LOWPOLY_PLAY_WINDOW_UV.into(),
        label: semio_framework_plugin::LocalizedLabel::native("UV", "UV"),
        body_key: LOWPOLY_PLAY_BODY_UV.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "layout-grid".into(),
        options: WindowOptions { measures: Vec::new(), engagement: WindowEngagementSlot::Some(engagement) },
        actions: Vec::new(),
        utilities: ["brush", "eraser", "fill", "eyedropper"].iter().map(|id| UtilityRef::from(*id)).collect(),
        // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the UV window paints textures —
        // it never selects/hovers mesh components, so it declares no interaction domain.
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

/// 🎚️ The live chrome measures for this window — identical set to the Model window (see the master
/// ticket's TEMPLATE.md §12.2 shared-options pattern).
pub fn window_measures(config: &LowpolyConfig, labels: &LowpolyLabels) -> Vec<WindowMeasure> {
    lowpoly_window_measures(config, labels)
}
//#endregion 🔖️Definition

//#region 🔖️Scene
/// 🎒️ `layers_json` rides the fixed-capacity Canvas2d spine (32 KiB, no paged lane), so the UV scene is
/// fitted to this budget: the wireframe first (clipped to whole segments), then the paint texture only
/// when it still fits. A 256² painted texture or a real mesh's UV net alone overflowed it
/// (`scene-surface.encode … 39040 bytes`, 2026-09-17), which stopped the whole actor.
const LOWPOLY_UV_LAYERS_BUDGET_BYTES: usize = 26 * 1024;

fn uv_canvas_layers_json(doc: &LowpolyDocument, view: LowpolyView<'_>, texture_cache: &HashMap<String, String>) -> String {
    use crate::editor::lowpoly::view::resolve_active_object_id;
    let object_id = doc.active_object_id().to_string();
    let object_id = if object_id.is_empty() { resolve_active_object_id(view.snapshot, view.config) } else { object_id };
    let scale = LOWPOLY_PAINT_TEXTURE_SIZE as f64;
    let mut wireframe: Option<dsl::DslValue> = None;
    if let Ok(mesh) = doc.active_mesh() {
        if let Ok(transfer) = LowpolyDocument::tessellate_transfer_json(mesh) {
            let edge_uvs: Vec<f32> = transfer.get("edgeUvs").and_then(|value| dsl::FromValue::from_value(value.clone()).ok()).unwrap_or_default();
            let edge_is_seam: Vec<u8> = transfer.get("edgeIsSeam").and_then(|value| dsl::FromValue::from_value(value.clone()).ok()).unwrap_or_default();
            // ✂️ ~48 printed bytes per segment (two rounded points + a seam flag).
            let segments = (LOWPOLY_UV_LAYERS_BUDGET_BYTES / 48).min(edge_uvs.len() / 4);
            let round = |value: f64| (value * 10.0).round() / 10.0;
            let mut points: Vec<[f64; 2]> = Vec::with_capacity(segments * 2);
            for chunk in edge_uvs.as_chunks::<4>().0.iter().take(segments) {
                points.push([round(chunk[0] as f64 * scale - scale * 0.5), round((1.0 - chunk[1]) as f64 * scale - scale * 0.5)]);
                points.push([round(chunk[2] as f64 * scale - scale * 0.5), round((1.0 - chunk[3]) as f64 * scale - scale * 0.5)]);
            }
            let seams: Vec<u8> = edge_is_seam.into_iter().take(segments).collect();
            wireframe = Some(dsl::DslValue::object([
                ("id".to_string(), dsl::DslValue::String("uv-wireframe".to_string())),
                ("kind".to_string(), dsl::DslValue::String("polyline".to_string())),
                ("name".to_string(), dsl::DslValue::String("UV Wireframe".to_string())),
                ("points".to_string(), dsl::ToValue::to_value(&points)),
                ("seams".to_string(), dsl::ToValue::to_value(&seams)),
            ]));
        }
    }
    let wireframe_bytes = wireframe.as_ref().map_or(0, |layer| dsl::json::to_json_string(layer).len());
    let mut layers: Vec<dsl::DslValue> = Vec::new();
    if let Some(texture) = texture_cache.get(&object_id).filter(|texture| texture.len() + wireframe_bytes + 256 <= LOWPOLY_UV_LAYERS_BUDGET_BYTES) {
        layers.push(dsl::DslValue::object([
            ("id".to_string(), dsl::DslValue::String("uv-paint-texture".to_string())),
            ("kind".to_string(), dsl::DslValue::String("image".to_string())),
            ("name".to_string(), dsl::DslValue::String("Paint".to_string())),
            ("x".to_string(), dsl::DslValue::float(-scale * 0.5)),
            ("y".to_string(), dsl::DslValue::float(-scale * 0.5)),
            ("width".to_string(), dsl::DslValue::float(scale)),
            ("height".to_string(), dsl::DslValue::float(scale)),
            ("dataUrl".to_string(), dsl::DslValue::String(format!("data:image/png;base64,{texture}"))),
        ]));
    }
    layers.extend(wireframe);
    dsl::json::to_json_string(&layers)
}

pub fn render(view: LowpolyView<'_>, loaded: Option<&LowpolyDocument>, texture_cache: &HashMap<String, String>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    match loaded {
        Some(loaded) => {
            scene_surface(LOWPOLY_PLAY_SURFACE_UV, semio_framework_ui_contract::SurfaceKind::Canvas2d, &Canvas2dScene { camera_x: 0.0, camera_y: 0.0, zoom: 1.0, layers_json: uv_canvas_layers_json(loaded, view, texture_cache), snapshot: None, tool_run_trace: None, lanes: Vec::new() })
        }
        None => semio_framework_plugin::built_text_node(semio_framework_plugin::Label::data("Failed to load UV canvas")).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly uv window failed-load text admission failed")),
    }
}
//#endregion 🔖️Scene

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
