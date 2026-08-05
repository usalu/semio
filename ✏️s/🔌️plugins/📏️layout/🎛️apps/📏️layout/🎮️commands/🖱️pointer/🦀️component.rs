//! 🖱️ Layout play app commands — canvas pointer/drag/camera interactions on the blueprint/preview
//! surfaces. `CanvasDrop` is the one document-mutating row here: it delegates to
//! `crate::apps::layout::commands::author::{add_frame, add_page}`'s own handlers so the "drop adds
//! content" behavior has exactly one implementation.

use crate::apps::layout::canvas::active_page;
use crate::apps::layout::commands::author::{add_frame, add_page};
use crate::apps::layout::config::{LayoutConfig, LayoutDropPreviewState};
use crate::apps::layout::config::LayoutConfigOperation;
use crate::artifacts::layout::engine::scene::build_display_list_for_page;
use crate::artifacts::layout::op::LayoutOperation;
use crate::artifacts::layout::{LayoutCamera, LayoutDocument};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Shared
/// 🖱️ A surface id names its blueprint/preview surface directly (`"layout.play.blueprint"` /
/// `"layout.play.preview"`); an absent id defaults to blueprint (the interactive authoring surface).
fn surface_is_blueprint(surface_id: Option<&str>) -> bool {
    surface_id.is_none_or(|surface| surface.contains("blueprint"))
}

fn screen_to_world_for_surface(config: &LayoutConfig, blueprint: bool, sx: f64, sy: f64, width: f64, height: f64) -> (f64, f64) {
    let camera_runtime = if blueprint { &config.camera } else { &config.preview_camera };
    let camera = infinite_canvas::camera::Camera { x: camera_runtime.x, y: camera_runtime.y, zoom: camera_runtime.zoom.max(0.0001) };
    let viewport = infinite_canvas::camera::Viewport { width: width.max(1.0) as u32, height: height.max(1.0) as u32, dpr: 1.0 };
    let world = infinite_canvas::camera::screen_to_world(&camera, &viewport, infinite_canvas::Point::new(sx, sy));
    (world.x, world.y)
}

#[allow(clippy::too_many_arguments)]
fn hit_test_at(doc: &LayoutDocument, config: &LayoutConfig, sx: f64, sy: f64, width: f64, height: f64, blueprint: bool) -> Option<String> {
    let page = active_page(doc, config)?;
    let (wx, wy) = screen_to_world_for_surface(config, blueprint, sx, sy, width, height);
    let list = build_display_list_for_page(doc, page, &page.id, &config.selected_ids, config.hovered_id.as_deref(), blueprint);
    list.hit_test(wx as f32, wy as f32)
}
//#endregion 🔖️Shared

//#region 🔖️CanvasPointerDown
pub mod canvas_pointer_down {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "canvas-pointer-down")]
    pub struct CanvasPointerDown {
        pub surface_id: Option<String>,
        pub button: i64,
        pub extend: bool,
        pub x: f64,
        pub y: f64,
        pub width: f64,
        pub height: f64,
    }

    pub fn handle(payload: &CanvasPointerDown, doc: &DocumentView<'_, LayoutDocument>, cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutOperation, LayoutConfigOperation>, Fault> {
        let blueprint = surface_is_blueprint(payload.surface_id.as_deref());
        if !blueprint || payload.button != 0 {
            return Ok(Emit::default());
        }
        let hit = hit_test_at(doc.projection, cfg.projection, payload.x, payload.y, payload.width, payload.height, blueprint);
        let ids = match hit {
            Some(id) if payload.extend => {
                let mut ids = cfg.projection.selected_ids.clone();
                if let Some(position) = ids.iter().position(|existing| *existing == id) {
                    ids.remove(position);
                } else {
                    ids.push(id);
                }
                ids
            }
            Some(id) => vec![id],
            None => Vec::new(),
        };
        Ok(Emit::config(vec![LayoutConfigOperation::SetSelection { ids }]))
    }
}
//#endregion 🔖️CanvasPointerDown

//#region 🔖️CanvasPointerMove
pub mod canvas_pointer_move {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "canvas-pointer-move")]
    pub struct CanvasPointerMove {
        pub surface_id: Option<String>,
        pub x: f64,
        pub y: f64,
        pub width: f64,
        pub height: f64,
    }

    pub fn handle(payload: &CanvasPointerMove, doc: &DocumentView<'_, LayoutDocument>, cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutOperation, LayoutConfigOperation>, Fault> {
        let blueprint = surface_is_blueprint(payload.surface_id.as_deref());
        if !blueprint {
            return Ok(Emit::default());
        }
        Ok(Emit::config(vec![LayoutConfigOperation::SetHover { id: hit_test_at(doc.projection, cfg.projection, payload.x, payload.y, payload.width, payload.height, blueprint) }]))
    }
}
//#endregion 🔖️CanvasPointerMove

//#region 🔖️CanvasPointerUp
pub mod canvas_pointer_up {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "canvas-pointer-up")]
    pub struct CanvasPointerUp {}

    pub fn handle(_payload: &CanvasPointerUp, _doc: &DocumentView<'_, LayoutDocument>, _cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutOperation, LayoutConfigOperation>, Fault> {
        Ok(Emit::default())
    }
}
//#endregion 🔖️CanvasPointerUp

//#region 🔖️CanvasDragOver
pub mod canvas_drag_over {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "canvas-drag-over")]
    pub struct CanvasDragOver {
        pub surface_id: Option<String>,
        pub kind: String,
        pub x: f64,
        pub y: f64,
        pub width: f64,
        pub height: f64,
    }

    pub fn handle(payload: &CanvasDragOver, _doc: &DocumentView<'_, LayoutDocument>, cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutOperation, LayoutConfigOperation>, Fault> {
        let blueprint = surface_is_blueprint(payload.surface_id.as_deref());
        if !blueprint {
            return Ok(Emit::default());
        }
        let (wx, wy) = screen_to_world_for_surface(cfg.projection, blueprint, payload.x, payload.y, payload.width, payload.height);
        Ok(Emit::config(vec![LayoutConfigOperation::SetDropPreview { preview: LayoutDropPreviewState { kind: payload.kind.clone(), x: wx, y: wy } }]))
    }
}
//#endregion 🔖️CanvasDragOver

//#region 🔖️CanvasDragLeave
pub mod canvas_drag_leave {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "canvas-drag-leave")]
    pub struct CanvasDragLeave {}

    pub fn handle(_payload: &CanvasDragLeave, _doc: &DocumentView<'_, LayoutDocument>, _cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutOperation, LayoutConfigOperation>, Fault> {
        Ok(Emit::config(vec![LayoutConfigOperation::SetDropPreview { preview: LayoutDropPreviewState::default() }]))
    }
}
//#endregion 🔖️CanvasDragLeave

//#region 🔖️SetCamera
pub mod set_camera {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "camera")]
    pub struct SetCamera {
        pub surface_id: Option<String>,
        #[dsl(block)]
        pub camera: LayoutCamera,
    }

    pub fn handle(payload: &SetCamera, _doc: &DocumentView<'_, LayoutDocument>, cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutOperation, LayoutConfigOperation>, Fault> {
        let blueprint = surface_is_blueprint(payload.surface_id.as_deref());
        let _ = cfg;
        if blueprint {
            Ok(Emit::config(vec![LayoutConfigOperation::SetCamera { camera: payload.camera.clone() }]))
        } else {
            Ok(Emit::config(vec![LayoutConfigOperation::SetPreviewCamera { camera: payload.camera.clone() }]))
        }
    }
}
//#endregion 🔖️SetCamera

//#region 🔖️CanvasDrop
pub mod canvas_drop {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "canvas-drop")]
    pub struct CanvasDrop {
        pub surface_id: Option<String>,
        pub kind: String,
        pub x: f64,
        pub y: f64,
        pub width: f64,
        pub height: f64,
    }

    /// 🐛️ Delegates document creation to `add_page`/`add_frame`'s own handlers so "drop adds content"
    /// has one implementation, then always clears the drag-ghost regardless of surface/outcome.
    pub fn handle(payload: &CanvasDrop, doc: &DocumentView<'_, LayoutDocument>, cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutOperation, LayoutConfigOperation>, Fault> {
        let blueprint = surface_is_blueprint(payload.surface_id.as_deref());
        if !blueprint {
            return Ok(Emit::config(vec![LayoutConfigOperation::SetDropPreview { preview: LayoutDropPreviewState::default() }]));
        }
        let (wx, wy) = screen_to_world_for_surface(cfg.projection, blueprint, payload.x, payload.y, payload.width, payload.height);
        let mut emitted = if payload.kind == "page" {
            add_page::handle(&add_page::AddPage {}, doc, cfg)?
        } else {
            add_frame::handle(&add_frame::AddFrame { kind: payload.kind.clone(), x: Some(wx), y: Some(wy) }, doc, cfg)?
        };
        emitted.config_operations.push(LayoutConfigOperation::SetDropPreview { preview: LayoutDropPreviewState::default() });
        Ok(emitted)
    }
}
//#endregion 🔖️CanvasDrop

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::layout::testkit::{dispatch, layout_app, render, test_screen_point};
    use crate::apps::layout::{LayoutCommand, LAYOUT_PLAY_BODY_DOCUMENT, LAYOUT_PLAY_SURFACE_BLUEPRINT, LAYOUT_PLAY_SURFACE_PREVIEW};

    #[test]
    fn set_camera_mutates_config_and_emits_no_operations() {
        let mut app = layout_app();
        let before = app.projection().expect("projection");
        let result = dispatch(&mut app, LayoutCommand::SetCamera(set_camera::SetCamera { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), camera: LayoutCamera { x: 10.0, y: 20.0, zoom: 1.5 } }));
        assert!(result.operations.is_empty(), "camera is a config action and emits no operations");
        assert_eq!(app.projection().expect("projection"), before, "camera never mutates the document");
    }

    #[test]
    fn set_camera_preview_surface_updates_independently_of_blueprint() {
        let mut app = layout_app();
        dispatch(&mut app, LayoutCommand::SetCamera(set_camera::SetCamera { surface_id: Some(LAYOUT_PLAY_SURFACE_PREVIEW.into()), camera: LayoutCamera { x: 3.0, y: 4.0, zoom: 2.0 } }));
        let preview_json = render(&mut app, crate::apps::layout::modes::edit::windows::preview::LAYOUT_PLAY_BODY_PREVIEW);
        assert!(preview_json.contains(r#""cameraX":3.0"#), "preview scene reflects config camera: {preview_json}");
        let blueprint_json = render(&mut app, crate::apps::layout::modes::edit::windows::blueprint::LAYOUT_PLAY_BODY_BLUEPRINT);
        assert!(blueprint_json.contains(r#""cameraX":0.0"#), "blueprint surface camera stays independent: {blueprint_json}");
    }

    #[test]
    fn pointer_down_selects_frame_via_hit_test() {
        let mut app = layout_app();
        let (sx, sy) = test_screen_point(0.0, 0.0, 1.0, 800.0, 600.0, 136.0, 435.0);
        dispatch(&mut app, LayoutCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), button: 0, extend: false, x: sx, y: sy, width: 800.0, height: 600.0 }));
        let json = render(&mut app, LAYOUT_PLAY_BODY_DOCUMENT);
        assert!(json.contains("layout-document.frame.frame-image-1"));
    }

    #[test]
    fn pointer_move_updates_hover_highlight() {
        let mut app = layout_app();
        let (sx, sy) = test_screen_point(0.0, 0.0, 1.0, 800.0, 600.0, 156.0, 220.0);
        let result = dispatch(&mut app, LayoutCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), x: sx, y: sy, width: 800.0, height: 600.0 }));
        assert!(result.operations.is_empty(), "hover is a config action, not an operation");
        let json = render(&mut app, LAYOUT_PLAY_BODY_DOCUMENT);
        assert!(json.contains("layout-document.frame.frame-text-1"));
    }

    #[test]
    fn canvas_drop_adds_frame_at_world_coords() {
        let mut app = layout_app();
        let (sx, sy) = test_screen_point(0.0, 0.0, 1.0, 800.0, 600.0, 100.0, 200.0);
        let result = dispatch(&mut app, LayoutCommand::CanvasDrop(canvas_drop::CanvasDrop { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), kind: "rect".into(), x: sx, y: sy, width: 800.0, height: 600.0 }));
        assert_eq!(result.operations.len(), 1);
        let doc = app.projection().expect("projection");
        let frame = doc.pages[0].frames.last().unwrap();
        let bounds = frame.bounds();
        assert!((bounds.x - 100.0).abs() < 0.01);
        assert!((bounds.y - 200.0).abs() < 0.01);
    }

    #[test]
    fn canvas_drop_page_kind_adds_page() {
        let mut app = layout_app();
        let before = app.projection().expect("projection").pages.len();
        let result = dispatch(&mut app, LayoutCommand::CanvasDrop(canvas_drop::CanvasDrop { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), kind: "page".into(), x: 0.0, y: 0.0, width: 800.0, height: 600.0 }));
        assert_eq!(result.operations.len(), 1);
        assert_eq!(app.projection().expect("projection").pages.len(), before + 1);
    }

    #[test]
    fn drag_over_emits_ghost_and_leave_clears() {
        let mut app = layout_app();
        dispatch(&mut app, LayoutCommand::CanvasDragOver(canvas_drag_over::CanvasDragOver { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), kind: "rect".into(), x: 400.0, y: 300.0, width: 800.0, height: 600.0 }));
        assert!(render(&mut app, crate::apps::layout::modes::edit::windows::blueprint::LAYOUT_PLAY_BODY_BLUEPRINT).contains("layout.drop-preview"));

        dispatch(&mut app, LayoutCommand::CanvasDragLeave(canvas_drag_leave::CanvasDragLeave {}));
        assert!(!render(&mut app, crate::apps::layout::modes::edit::windows::blueprint::LAYOUT_PLAY_BODY_BLUEPRINT).contains("layout.drop-preview"));
    }
}
//#endregion 🧪️Tests
