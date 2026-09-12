//! 📊️ DIN V 18599 play app — the results window: every computed compliance check, one line each.

use crate::document::NormHost;
use crate::editor::din18599::DinV18599Family;
use semio_framework_plugin::{LocalizedLabel, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_RESULTS: &str = "norm-din18599-results";
crate::norm_results_window_config_owner!(ResultsWindowConfigOwner, WINDOW_RESULTS);

pub const BODY_RESULTS: &str = "norm.din18599.play.results";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::din18599::create_din18599_app`.
pub fn definition() -> WindowKindDefinition {
    crate::app_surface::window_definition(WINDOW_RESULTS, LocalizedLabel::native("Results", "Ergebnisse"), BODY_RESULTS, "bar-chart-3")
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(host: &NormHost<DinV18599Family>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    crate::app_surface::render_report(host.report())
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
