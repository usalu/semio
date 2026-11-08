//! 📈️ VDI 3805 viewer — the read-only diagrams window; imports nothing from the editor (`policyViewerPurityBreaches`).

use semio_plugin::{BuiltNode, TreeWindows, UiAssemblyResult, WindowKindDefinition};
use semio_ui_locale::{Locale, LocalizedLabel};
use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;
use semio_s_artifact_norm_vdi3805_scene::diagrams;
use semio_s_plugin_norm_structural_scene::projection::memo_diagrams;
use semio_s_plugin_norm_app_surface::{render_diagrams, DiagramsWindowSpec};

//#region 🔖️Constants
pub const WINDOW_DIAGRAMS: &str = "norm-vdi3805-diagrams";
pub const BODY_DIAGRAMS: &str = "norm.vdi3805.view.diagrams";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::create_vdi3805_viewer`.
pub fn definition() -> WindowKindDefinition {
    semio_s_plugin_norm_app_surface::diagram_window_definition("vdi3805", LocalizedLabel::native("Diagrams", "Diagramme"), BODY_DIAGRAMS, "chart-line")
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 📈️ Every diagram of the document.
pub fn render(document: &Vdi3805Snapshot, windows: &TreeWindows<'_>, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let report = semio_s_artifact_norm_vdi3805_inferences::live::report(document);
    let shown = memo_diagrams("vdi3805", document, &report, "\u{0}", || diagrams(document, &report, None));
    render_diagrams(BODY_DIAGRAMS, &shown, &DiagramsWindowSpec::default(), windows, locale)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
