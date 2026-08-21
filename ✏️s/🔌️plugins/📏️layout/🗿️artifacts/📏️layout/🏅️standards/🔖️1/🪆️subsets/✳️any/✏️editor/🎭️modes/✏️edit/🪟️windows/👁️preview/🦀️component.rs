//! 👁️ Layout play app — the Preview window: a read-only render of the current page with no chrome
//! (no guides, no dashed inherited-frame strokes) and its own independent camera pose.

use crate::artifacts::layout::LayoutSnapshot;
use crate::editor::layout::canvas::canvas_layers;
use crate::editor::layout::config::LayoutConfig;
use crate::editor::layout::LAYOUT_PLAY_APP_ID;
use semio_framework_plugin::{build_canvas_2d_scene, Canvas2dScene, LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const LAYOUT_PLAY_WINDOW_PREVIEW: &str = "layout-preview";
pub const LAYOUT_PLAY_BODY_PREVIEW: &str = "layout.play.preview";
pub const LAYOUT_PLAY_SURFACE_PREVIEW: &str = "layout.play.preview";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::layout::create_layout_app`. `options.measures`
/// stays empty: layout declares no config-derived chrome measures for this window.
pub async fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: LAYOUT_PLAY_WINDOW_PREVIEW.into(),
        label: LocalizedLabel::native("Preview", "Vorschau"),
        body_key: LAYOUT_PLAY_BODY_PREVIEW.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "preview".into(),
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
    let camera = &config.preview_camera;
    build_canvas_2d_scene(LAYOUT_PLAY_SURFACE_PREVIEW, LAYOUT_PLAY_APP_ID, Canvas2dScene { camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom, layers_json: canvas_layers(engine, doc, config, false) })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::layout::testkit::{layout_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn renders_preview_canvas_scene() {
        let mut app = layout_app();
        assert!(render_body(&mut app, LAYOUT_PLAY_BODY_PREVIEW).contains("canvas-2d"));
    }

    #[semio_framework_async_macros::async_test]
    async fn preview_scene_has_white_background_and_no_guides() {
        let mut app = layout_app();
        let json = render_body(&mut app, LAYOUT_PLAY_BODY_PREVIEW);
        assert!(json.contains("layout.page-bg"));
        assert!(!json.contains("layout.guide."));
    }

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_canvas_2d_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, LAYOUT_PLAY_BODY_PREVIEW);
        assert!(matches!(definition.surface_kind, SurfaceKind::Canvas2d));
        assert!(definition.options.measures.is_empty());
    }
}
//#endregion 🧪️Tests
