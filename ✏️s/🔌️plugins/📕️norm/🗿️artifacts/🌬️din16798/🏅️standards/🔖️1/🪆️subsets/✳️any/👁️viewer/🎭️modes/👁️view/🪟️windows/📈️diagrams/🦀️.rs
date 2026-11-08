//! 📈️ DIN EN 16798 viewer — the read-only diagrams window; imports nothing from the editor (`policyViewerPurityBreaches`).

use semio_plugin::{BuiltNode, TreeWindows, UiAssemblyResult, WindowKindDefinition};
use semio_ui_locale::{Locale, LocalizedLabel};
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
use semio_s_artifact_norm_din16798_scene::diagrams;
use semio_s_plugin_norm_structural_scene::projection::memo_diagrams;
use semio_s_plugin_norm_app_surface::{render_diagrams, DiagramsWindowSpec};

//#region 🔖️Constants
pub const WINDOW_DIAGRAMS: &str = "norm-din16798-diagrams";
pub const BODY_DIAGRAMS: &str = "norm.din16798.view.diagrams";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::create_din16798_viewer`.
pub fn definition() -> WindowKindDefinition {
    semio_s_plugin_norm_app_surface::diagram_window_definition("din16798", LocalizedLabel::native("Diagrams", "Diagramme"), BODY_DIAGRAMS, "chart-line")
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 📈️ Every diagram of the document.
pub fn render(document: &Din16798Snapshot, windows: &TreeWindows<'_>, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let report = semio_s_plugin_norm_app_surface::paint_report_of(|budget| semio_s_artifact_norm_din16798_inferences::live::responsive(document, budget));
    let shown = memo_diagrams("din16798", document, &report, "\u{0}", || diagrams(document, &report, None));
    render_diagrams(BODY_DIAGRAMS, &shown, &DiagramsWindowSpec::default(), windows, locale)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
