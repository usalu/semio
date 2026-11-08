//! 📈️ EN 1996 viewer — the read-only diagrams window; imports nothing from the editor (`policyViewerPurityBreaches`).

use semio_plugin::{BuiltNode, TreeWindows, UiAssemblyResult, WindowKindDefinition};
use semio_ui_locale::{Locale, LocalizedLabel};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;
use semio_s_artifact_norm_en1996_scene::diagrams;
use semio_s_plugin_norm_structural_scene::projection::memo_diagrams;
use semio_s_plugin_norm_app_surface::{render_diagrams, DiagramsWindowSpec};

//#region 🔖️Constants
pub const WINDOW_DIAGRAMS: &str = "norm-en1996-diagrams";
pub const BODY_DIAGRAMS: &str = "norm.en1996.view.diagrams";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::create_en1996_viewer`.
pub fn definition() -> WindowKindDefinition {
    semio_s_plugin_norm_app_surface::diagram_window_definition("en1996", LocalizedLabel::native("Diagrams", "Diagramme"), BODY_DIAGRAMS, "chart-line")
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 📈️ Every diagram of the document.
pub fn render(document: &En1996Snapshot, windows: &TreeWindows<'_>, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let report = semio_s_artifact_norm_en1996_inferences::live::responsive(document, semio_s_plugin_norm_app_surface::PAINT_BUDGET).0;
    let shown = memo_diagrams("en1996", document, &report, "\u{0}", || diagrams(document, &report, None));
    render_diagrams(BODY_DIAGRAMS, &shown, &DiagramsWindowSpec::default(), windows, locale)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
