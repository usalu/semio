//! 📊️ VDI 3805 play app — the results window: every computed compliance check, one line each.

use semio_s_plugin_norm_compliance::NormHost;
use crate::Vdi3805Family;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::WindowKindDefinition;

//#region 🔖️Constants
pub const WINDOW_RESULTS: &str = "norm-vdi3805-results";
semio_s_plugin_norm_results_config::norm_results_window_config_owner!(ResultsWindowConfigOwner, WINDOW_RESULTS);

pub const BODY_RESULTS: &str = "norm.vdi3805.play.results";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::vdi3805::create_vdi3805_app`.
pub fn definition() -> WindowKindDefinition {
    semio_s_plugin_norm_app_surface::window_definition(WINDOW_RESULTS, LocalizedLabel::native("Results", "Ergebnisse"), BODY_RESULTS, "bar-chart-3")
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(host: &NormHost<Vdi3805Family>, windows: &semio_framework_plugin::TreeWindows<'_>, locale: semio_framework_ui_locale::Locale, controller_id: Option<&'static str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    semio_s_plugin_norm_app_surface::render_report(host.report(), windows, locale, controller_id)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
