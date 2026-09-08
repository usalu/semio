//! 📐️ Equation play app — the geometry window: the convex-hull/centroid canvas.

use crate::EquationGeometry;
use crate::editor::equation::geometry_layers_json;
use semio_framework_plugin::{Canvas2dScene, LocalizedLabel, SurfaceKind, BuiltNode, UiAssemblyResult, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const MATH_PLAY_WINDOW_GEOMETRY: &str = "math-geometry";
pub const MATH_PLAY_BODY_GEOMETRY: &str = "equation.play.geometry";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::equation::create_equation_app`.
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
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
        interactions: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(geometry: &EquationGeometry) -> UiAssemblyResult<BuiltNode> {
    semio_framework_plugin::scene_surface(MATH_PLAY_BODY_GEOMETRY, semio_framework_plugin::plugin_app_close_prelude::SurfaceKind::Canvas2d, &Canvas2dScene { camera_x: 0.0, camera_y: 0.0, zoom: 1.0, layers_json: geometry_layers_json(geometry), snapshot: None })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
