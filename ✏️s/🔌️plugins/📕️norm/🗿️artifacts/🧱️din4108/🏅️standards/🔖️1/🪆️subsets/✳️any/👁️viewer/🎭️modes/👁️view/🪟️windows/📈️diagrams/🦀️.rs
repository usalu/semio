//! 📈️ DIN 4108 viewer — the read-only diagrams window; imports nothing from the editor (`policyViewerPurityBreaches`).

use semio_plugin::{BuiltNode, TreeWindows, UiAssemblyResult, WindowKindDefinition};
use semio_ui_locale::{Locale, LocalizedLabel};
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;
use semio_s_artifact_norm_din4108_scene::diagrams;
use semio_s_plugin_norm_structural_scene::projection::memo_diagrams;
use semio_s_plugin_norm_app_surface::{render_diagrams, DiagramsWindowSpec};

//#region 🔖️Constants
pub const WINDOW_DIAGRAMS: &str = "norm-din4108-diagrams";
pub const BODY_DIAGRAMS: &str = "norm.din4108.view.diagrams";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::create_din4108_viewer`.
pub fn definition() -> WindowKindDefinition {
    semio_s_plugin_norm_app_surface::diagram_window_definition("din4108", LocalizedLabel::native("Diagrams", "Diagramme"), BODY_DIAGRAMS, "chart-line")
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 📈️ Every diagram of the document.
pub fn render(document: &Din4108Snapshot, windows: &TreeWindows<'_>, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let report = semio_s_plugin_norm_app_surface::paint_report::<crate::Din4108ViewFamily>(document);
    let shown = memo_diagrams("din4108", document, &report, "\u{0}", || diagrams(document, &report, None));
    render_diagrams(BODY_DIAGRAMS, &shown, &DiagramsWindowSpec::default(), windows, locale)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
