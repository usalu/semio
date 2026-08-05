//! 📐️ Mathematical play app — the geometry window: the convex-hull/centroid canvas.

use crate::artifacts::mathematical::engine::{empty_component_scene, geometry_layers_json};
use crate::artifacts::mathematical::MathGeometry;
use semio_framework_plugin::{Canvas2dScene, LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const MATH_PLAY_WINDOW_GEOMETRY: &str = "math-geometry";
pub const MATH_PLAY_BODY_GEOMETRY: &str = "mathematical.play.geometry";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::apps::mathematical::create_mathematical_app`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: MATH_PLAY_WINDOW_GEOMETRY.into(),
        label: LocalizedLabel::native("Geometry", "Geometrie"),
        body_key: MATH_PLAY_BODY_GEOMETRY.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "hexagon".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        document_projection_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(geometry: &MathGeometry) -> UiNode {
    let mut scene = empty_component_scene(MATH_PLAY_BODY_GEOMETRY, SurfaceKind::Canvas2d);
    scene.canvas_2d = Some(Canvas2dScene { camera_x: 0.0, camera_y: 0.0, zoom: 1.0, layers_json: geometry_layers_json(geometry) });
    UiNode::ComponentScene(scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_canvas_2d_scene() {
        let json = serde_json::to_string(&render(&MathGeometry::default())).unwrap();
        assert!(json.contains("canvas-2d"));
    }

    #[test]
    fn definition_declares_the_canvas_2d_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, MATH_PLAY_BODY_GEOMETRY);
        assert!(matches!(definition.surface_kind, SurfaceKind::Canvas2d));
    }
}
//#endregion 🧪️Tests
