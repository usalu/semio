//! ✅️ EN 1999 play app — the checks window: every computed compliance check with utilisation bars, clause links and remedy chips.

use semio_plugin::{BuiltNode, TreeWindows, UiAssemblyResult, WindowKindDefinition};
use semio_ui_locale::{Locale, LocalizedLabel};
use semio_s_plugin_norm_app_surface::{render_checks_with, ChecksViewState, ChecksWindowSpec};
use semio_s_plugin_norm_compliance::CheckReport;

//#region 🔖️Constants
pub const WINDOW_CHECKS: &str = "norm-en1999-checks";
semio_s_plugin_norm_results_config::norm_results_window_config_owner!(ChecksWindowConfigOwner, WINDOW_CHECKS);

pub const BODY_CHECKS: &str = "norm.en1999.play.checks";
pub const INTERACTION_DOMAIN: &str = "norm.en1999";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::create_en1999_app`.
pub fn definition() -> WindowKindDefinition {
    semio_s_plugin_norm_app_surface::window_definition(WINDOW_CHECKS, LocalizedLabel::native("Checks", "Nachweise"), BODY_CHECKS, "check-circle")
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(report: &CheckReport, selected: Option<u32>, windows: &TreeWindows<'_>, locale: Locale, controller_id: Option<&'static str>) -> UiAssemblyResult<BuiltNode> {
    let state = ChecksViewState { selected: selected.map(|index| index as usize), ..ChecksViewState::default() };
    render_checks_with(report, &state, &ChecksWindowSpec { controller_id, interaction_domain: Some(INTERACTION_DOMAIN), ..ChecksWindowSpec::default() }, windows, locale)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
