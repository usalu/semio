//! 📐️ Layout play app — the Blueprint window: the editable authoring surface with chrome (guides,
//! margins, dashed inherited-frame strokes) — the only window content-authoring actions are scoped to.

use crate::editor::layout::canvas::canvas_layers;
use crate::editor::layout::config::LayoutConfig;
use crate::editor::layout::LAYOUT_PLAY_APP_ID;
use crate::artifacts::layout::LayoutSnapshot;
use semio_framework_plugin::{build_canvas_2d_scene, Canvas2dScene, LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const LAYOUT_PLAY_WINDOW_BLUEPRINT: &str = "layout-blueprint";
pub const LAYOUT_PLAY_BODY_BLUEPRINT: &str = "layout.play.blueprint";
pub const LAYOUT_PLAY_SURFACE_BLUEPRINT: &str = "layout.play.blueprint";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::layout::create_layout_app`. `options.measures`
/// stays empty: layout declares no config-derived chrome measures for this window.
pub async fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: LAYOUT_PLAY_WINDOW_BLUEPRINT.into(),
        label: LocalizedLabel::native("Blueprint", "Entwurf"),
        body_key: LAYOUT_PLAY_BODY_BLUEPRINT.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "layout".into(),
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
pub async fn render(engine: &mut crate::editor::layout::engine::scene::LayoutEngine, doc: &LayoutSnapshot, config: &LayoutConfig) -> UiNode {
    let camera = &config.camera;
    build_canvas_2d_scene(LAYOUT_PLAY_SURFACE_BLUEPRINT, LAYOUT_PLAY_APP_ID, Canvas2dScene { camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom, layers_json: canvas_layers(engine, doc, config, true) })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::layout::testkit::{layout_app, render as render_body};

    #[test]
    async fn renders_blueprint_canvas_scene() {
        let mut app = layout_app();
        assert!(render_body(&mut app, LAYOUT_PLAY_BODY_BLUEPRINT).contains("canvas-2d"));
    }

    #[test]
    async fn blueprint_scene_has_page_background_and_guides() {
        // 🧷️ `layers_json` is a `String` field (`Canvas2dScene.layers_json`), so the render's own JSON
        // encoding escapes its embedded quotes — assert on the unquoted substrings that survive either
        // way rather than on an exact `"key":"value"` shape.
        let mut app = layout_app();
        let json = render_body(&mut app, LAYOUT_PLAY_BODY_BLUEPRINT);
        assert!(json.contains("layout.page-bg"));
        assert!(json.contains("0.97"));
        assert!(json.contains("layout.guide.margin"));
        assert!(json.contains("layout.guide.column"));
        assert!(json.contains("segments"));
        assert!(json.contains("fill") && json.contains("color"));
        assert!(!json.contains("linkId"));
    }

    #[test]
    async fn inherited_frame_gets_dashed_stroke_in_blueprint() {
        let mut app = layout_app();
        let json = render_body(&mut app, LAYOUT_PLAY_BODY_BLUEPRINT);
        assert!(json.contains("dash") && json.contains("4.0") && json.contains("3.0"));
    }

    #[test]
    async fn definition_declares_the_canvas_2d_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, LAYOUT_PLAY_BODY_BLUEPRINT);
        assert!(matches!(definition.surface_kind, SurfaceKind::Canvas2d));
        assert!(definition.options.measures.is_empty());
    }
}
//#endregion 🧪️Tests
