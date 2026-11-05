//! ✅️ EN 1999 viewer — the read-only checks window (no actions, no remedies applied); imports nothing from the editor (`policyViewerPurityBreaches`).

use semio_plugin::{BuiltNode, TreeWindows, UiAssemblyResult, WindowKindDefinition};
use semio_ui_locale::{Locale, LocalizedLabel};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_plugin_norm_app_surface::{render_checks_with, ChecksViewState, ChecksWindowSpec};

//#region 🔖️Constants
pub const WINDOW_CHECKS: &str = "norm-en1999-checks";
pub const BODY_CHECKS: &str = "norm.en1999.view.checks";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::create_en1999_viewer`.
pub fn definition() -> WindowKindDefinition {
    semio_s_plugin_norm_app_surface::window_definition(WINDOW_CHECKS, LocalizedLabel::native("Checks", "Nachweise"), BODY_CHECKS, "check-circle")
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// ✅️ The settled report of the document, rendered without any controller so no row dispatches a command.
pub fn render(document: &En1999Snapshot, windows: &TreeWindows<'_>, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let report = semio_s_artifact_norm_en1999_inferences::live::report(document);
    render_checks_with(&report, &ChecksViewState::default(), &ChecksWindowSpec::default(), windows, locale)
}

/// ✅️ The settled report with the check the view lane selected (stepping), still without any controller.
pub fn render_selected(document: &En1999Snapshot, selected: Option<usize>, windows: &TreeWindows<'_>, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let report = semio_s_artifact_norm_en1999_inferences::live::report(document);
    render_checks_with(&report, &ChecksViewState { selected, ..ChecksViewState::default() }, &ChecksWindowSpec::default(), windows, locale)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
