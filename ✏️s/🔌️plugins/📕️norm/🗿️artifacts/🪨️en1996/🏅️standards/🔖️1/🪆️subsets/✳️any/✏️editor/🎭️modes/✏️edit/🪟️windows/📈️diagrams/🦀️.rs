//! 📈️ EN 1996 play app — the diagrams window: eccentricity, Φ, N_Ed against N_Rd, slenderness, f_k, shear, lateral α and capacity, fire thickness and M–N diagrams of the selected check's wall.

use semio_plugin::{BuiltNode, TreeWindows, UiAssemblyResult, WindowKindDefinition};
use semio_ui_locale::{Locale, LocalizedLabel};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;
use semio_s_artifact_norm_en1996_scene_diagrams::diagrams;
use semio_s_plugin_norm_app_surface::{render_diagrams, DiagramsWindowSpec};
use semio_s_plugin_norm_compliance::CheckReport;
use semio_s_plugin_norm_structural_scene::projection::memo_diagrams;

//#region 🔖️Constants
pub const WINDOW_DIAGRAMS: &str = "norm-en1996-diagrams";
pub const BODY_DIAGRAMS: &str = "norm.en1996.play.diagrams";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::create_en1996_app`.
pub fn definition() -> WindowKindDefinition {
    semio_s_plugin_norm_app_surface::diagram_window_definition(crate::VARIANT, LocalizedLabel::native("Diagrams", "Diagramme"), BODY_DIAGRAMS, "chart-line")
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 📈️ Every diagram of the document (no focus), the set the editable marker rows of the window commit against.
pub fn all(document: &En1996Snapshot, report: &CheckReport) -> Vec<semio_s_plugin_norm_structural_scene::Diagram2d> {
    memo_diagrams(crate::VARIANT, document, report, "\u{0}", || diagrams(document, report, None)).to_vec()
}

/// 📈️ The diagrams of `focus`; every diagram when nothing is selected or the focus has none; each with screen-reader text and table twin.
pub fn render(document: &En1996Snapshot, report: &CheckReport, focus: Option<&str>, windows: &TreeWindows<'_>, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let of = |focus: Option<&str>| memo_diagrams(crate::VARIANT, document, report, focus.unwrap_or("\u{0}"), || diagrams(document, report, focus));
    let focused = of(focus);
    let shown = if focused.is_empty() { of(None) } else { focused };
    render_diagrams(BODY_DIAGRAMS, &shown, &DiagramsWindowSpec { controller_id: Some(crate::CONTROLLER_ID), ..DiagramsWindowSpec::default() }, windows, locale)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
