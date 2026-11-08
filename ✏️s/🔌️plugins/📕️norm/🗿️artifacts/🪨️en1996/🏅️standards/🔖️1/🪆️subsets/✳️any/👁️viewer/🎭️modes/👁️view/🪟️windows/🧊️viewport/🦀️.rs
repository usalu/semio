//! 🧊️ EN 1996 viewer — the read-only 3D model window over the same projected scene the editor shows; imports nothing from the editor (`policyViewerPurityBreaches`).

use semio_plugin::{BuiltNode, UiAssemblyResult, WindowKindDefinition};
use semio_ui_locale::{Locale, LocalizedLabel};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;
use semio_s_artifact_norm_en1996_scene::live_scene_of;
use semio_s_plugin_norm_app_surface::viewport3d::toolbar::{render_viewport_panel, ViewportView};
use semio_s_plugin_norm_app_surface::interaction_domain_id;

//#region 🔖️Constants
pub const WINDOW_VIEWPORT: &str = "norm-en1996-viewport";
pub const CONTROLLER: &str = "s.norm.en1996@1/*#viewer";
pub const BODY_VIEWPORT: &str = "norm.en1996.view.viewport";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::create_en1996_viewer`.
pub fn definition() -> WindowKindDefinition {
    semio_s_plugin_norm_app_surface::viewport_window_definition("en1996", LocalizedLabel::native("Model", "Modell"), BODY_VIEWPORT, "box")
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🧊️ Pure `En1996Snapshot -> world-3d surface` read.
pub fn render(document: &En1996Snapshot, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    render_with(document, &ViewportView::default(), locale)
}

/// 🎞️ The same window at an explicit view: the persisted local-only configuration and the ephemeral playback state, with the field picker and playback toolbar above the surface.
pub fn render_with(document: &En1996Snapshot, view: &ViewportView, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let report = semio_s_artifact_norm_en1996_inferences::live::responsive(document, semio_s_plugin_norm_app_surface::PAINT_BUDGET).0;
    let scene = live_scene_of(document, &report);
    render_viewport_panel(BODY_VIEWPORT, &scene, view, &[], &interaction_domain_id("en1996"), CONTROLLER, locale)
}

/// 🎞️ The same window at the ephemeral local view lane of the viewer (colour field, track, value, playback).
pub fn render_lane(document: &En1996Snapshot, lane: &semio_s_plugin_norm_app_surface::ViewLane, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    render_with(document, &ViewportView { lane: lane.clone(), ..ViewportView::default() }, locale)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
