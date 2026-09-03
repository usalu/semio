//! 📐️ Equation play app — the geometry window: the convex-hull/centroid canvas.

use crate::artifacts::equation::EquationGeometry;
use crate::editor::equation::{empty_component_scene, geometry_layers_json};
use semio_framework_plugin::{Canvas2dScene, LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const MATH_PLAY_WINDOW_GEOMETRY: &str = "math-geometry";
pub const MATH_PLAY_BODY_GEOMETRY: &str = "equation.play.geometry";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::equation::create_equation_app`.
pub async fn definition() -> WindowKindDefinition {
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
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
        interactions: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub async fn render(geometry: &EquationGeometry) -> UiNode {
    let mut scene = empty_component_scene(MATH_PLAY_BODY_GEOMETRY, SurfaceKind::Canvas2d);
    scene.canvas_2d = Some(Canvas2dScene { camera_x: 0.0, camera_y: 0.0, zoom: 1.0, layers_json: geometry_layers_json(geometry), snapshot: None });
    UiNode::ComponentScene(scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn renders_canvas_2d_scene() {
        // 🌱️ `UiNode` (`semio-framework-plugin`, framework-owned) has not itself gained `ToValue` —
        // `Debug` gives the same "the scene populated its canvas_2d slot" check the old JSON
        // substring check made, without needing `serde_json` for a framework type.
        let debug = format!("{:?}", render(&EquationGeometry::default()));
        assert!(debug.contains("canvas_2d: Some"), "expected a populated canvas_2d slot: {debug}");
    }

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_canvas_2d_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, MATH_PLAY_BODY_GEOMETRY);
        assert!(matches!(definition.surface_kind, SurfaceKind::Canvas2d));
    }
}
//#endregion 🧪️Tests
