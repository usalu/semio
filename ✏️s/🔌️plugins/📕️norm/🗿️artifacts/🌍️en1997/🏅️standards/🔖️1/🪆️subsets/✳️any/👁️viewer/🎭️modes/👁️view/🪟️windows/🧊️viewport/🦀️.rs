//! 🧊️ EN 1997 viewer — the read-only 3D model window over the same projected scene the editor shows; imports nothing from the editor (`policyViewerPurityBreaches`).

use semio_plugin::{BuiltNode, UiAssemblyResult, WindowKindDefinition};
use semio_ui_locale::{Locale, LocalizedLabel};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_scene::live_scene_of;
use semio_s_plugin_norm_app_surface::viewport3d::toolbar::{render_viewport_panel, ViewportView};
use semio_s_plugin_norm_app_surface::interaction_domain_id;

//#region 🔖️Constants
pub const WINDOW_VIEWPORT: &str = "norm-en1997-viewport";
pub const CONTROLLER: &str = "s.norm.en1997@1/*#viewer";
pub const BODY_VIEWPORT: &str = "norm.en1997.view.viewport";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::create_en1997_viewer`.
pub fn definition() -> WindowKindDefinition {
    semio_s_plugin_norm_app_surface::viewport_window_definition("en1997", LocalizedLabel::native("Model", "Modell"), BODY_VIEWPORT, "box")
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🧊️ Pure `En1997Snapshot -> world-3d surface` read.
pub fn render(document: &En1997Snapshot, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    render_with(document, &ViewportView::default(), locale)
}

/// 🎞️ The same window at an explicit view: the persisted local-only configuration and the ephemeral playback state, with the field picker and playback toolbar above the surface.
pub fn render_with(document: &En1997Snapshot, view: &ViewportView, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let report = semio_s_artifact_norm_en1997_inferences::live::report(document);
    let scene = live_scene_of(document, &report);
    render_viewport_panel(BODY_VIEWPORT, &scene, view, &[], &interaction_domain_id("en1997"), CONTROLLER, locale)
}

/// 🎞️ The same window at the ephemeral local view lane of the viewer (colour field, track, value, playback).
pub fn render_lane(document: &En1997Snapshot, lane: &semio_s_plugin_norm_app_surface::ViewLane, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    render_with(document, &ViewportView { lane: lane.clone(), ..ViewportView::default() }, locale)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
