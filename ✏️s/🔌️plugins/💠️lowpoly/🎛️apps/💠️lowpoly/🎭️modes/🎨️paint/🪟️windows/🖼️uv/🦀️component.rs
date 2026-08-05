//! 🖼️ Lowpoly play app — the UV window: the 2D UV-canvas paint surface. Only the paint operations it
//! shares with the Model window are scoped here (no mesh-editing/transform ops).

use crate::apps::lowpoly::config::LowpolyConfig;
use crate::apps::lowpoly::modes::edit::windows::model::LOWPOLY_TRANSFORM_UTILITY_DEFAULT;
use crate::apps::lowpoly::terminology::LowpolyLabels;
use crate::apps::lowpoly::view::LowpolyView;
use crate::apps::lowpoly::{lowpoly_window_engagement, lowpoly_window_measures};
use crate::artifacts::lowpoly::engine::LowpolyDocument;
use crate::artifacts::lowpoly::LOWPOLY_PAINT_TEXTURE_SIZE;
use semio_framework_plugin::{build_canvas_2d_scene, ActionRef, Canvas2dScene, SurfaceKind, UiNode, UtilityRef, WindowEngagementSlot, WindowKindDefinition, WindowMeasure, WindowOptions};
use serde_json::json;
use std::collections::HashMap;

//#region 🔖️Constants
pub const LOWPOLY_PLAY_WINDOW_UV: &str = "lowpoly-uv";
pub const LOWPOLY_PLAY_BODY_UV: &str = "lowpoly.play.uv";
const LOWPOLY_PLAY_SURFACE_UV: &str = "lowpoly.play.uv";

pub const LOWPOLY_UV_ACTIONS: &[&str] = &["addPaintLayer", "paintStrokeEnd", "paintFill", "fillBucket"];
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::apps::lowpoly::create_lowpoly_app`.
pub fn definition() -> WindowKindDefinition {
    let projection = crate::artifacts::lowpoly::engine::default_projection();
    let config = LowpolyConfig::default();
    let labels = semio_framework_plugin::resolve_labels_for_locale::<LowpolyLabels>("en-US");
    let engagement = lowpoly_window_engagement(LowpolyView { projection: &projection, config: &config }, LOWPOLY_TRANSFORM_UTILITY_DEFAULT, labels);
    WindowKindDefinition {
        id: LOWPOLY_PLAY_WINDOW_UV.into(),
        label: semio_framework_plugin::LocalizedLabel::native("UV", "UV"),
        body_key: LOWPOLY_PLAY_BODY_UV.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "layout-grid".into(),
        options: WindowOptions { measures: Vec::new(), engagement: WindowEngagementSlot::Some(engagement) },
        actions: LOWPOLY_UV_ACTIONS.iter().map(|id| ActionRef::from(*id)).collect(),
        utilities: ["brush", "eraser", "fill", "eyedropper"].iter().map(|id| UtilityRef::from(*id)).collect(),
        params_schema: None,
        document_projection_schema: None,
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
fn uv_canvas_layers_json(doc: &LowpolyDocument, view: LowpolyView<'_>, texture_cache: &HashMap<String, String>) -> String {
    use crate::apps::lowpoly::view::resolve_active_object_id;
    let object_id = resolve_active_object_id(view.projection, view.config);
    let mut layers = Vec::new();
    if let Some(texture) = texture_cache.get(&object_id) {
        let size = LOWPOLY_PAINT_TEXTURE_SIZE as f64;
        layers.push(json!({
            "id": "uv-paint-texture",
            "kind": "image",
            "name": "Paint",
            "x": -size * 0.5,
            "y": -size * 0.5,
            "width": size,
            "height": size,
            "dataUrl": format!("data:image/png;base64,{texture}"),
        }));
    }
    if let Ok(mesh) = doc.active_mesh() {
        if let Ok(transfer) = LowpolyDocument::tessellate_transfer_json(mesh) {
            let edge_uvs: Vec<f32> = transfer.get("edgeUvs").and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
            let edge_is_seam: Vec<u8> = transfer.get("edgeIsSeam").and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
            let mut points = Vec::new();
            for chunk in edge_uvs.as_chunks::<4>().0 {
                let u0 = chunk[0] as f64;
                let v0 = (1.0 - chunk[1]) as f64;
                let u1 = chunk[2] as f64;
                let v1 = (1.0 - chunk[3]) as f64;
                let scale = LOWPOLY_PAINT_TEXTURE_SIZE as f64;
                points.push([u0 * scale - scale * 0.5, v0 * scale - scale * 0.5]);
                points.push([u1 * scale - scale * 0.5, v1 * scale - scale * 0.5]);
            }
            layers.push(json!({
                "id": "uv-wireframe",
                "kind": "polyline",
                "name": "UV Wireframe",
                "points": points,
                "seams": edge_is_seam,
            }));
        }
    }
    serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into())
}

pub fn render(view: LowpolyView<'_>, loaded: Option<&LowpolyDocument>, texture_cache: &HashMap<String, String>) -> UiNode {
    match loaded {
        Some(loaded) => build_canvas_2d_scene(LOWPOLY_PLAY_SURFACE_UV, crate::apps::lowpoly::LOWPOLY_PLAY_APP_ID, Canvas2dScene { camera_x: 0.0, camera_y: 0.0, zoom: 1.0, layers_json: uv_canvas_layers_json(loaded, view, texture_cache) }),
        None => semio_framework_plugin::ui_text(semio_framework_plugin::Label::data("Failed to load UV canvas")),
    }
}
//#endregion 🔖️Scene

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::apps::lowpoly::testkit::{app, render};

    #[test]
    fn renders_uv_canvas() {
        let mut a = app();
        assert!(render(&mut a, super::LOWPOLY_PLAY_BODY_UV).contains("canvas-2d"));
    }
}
//#endregion 🧪️Tests
