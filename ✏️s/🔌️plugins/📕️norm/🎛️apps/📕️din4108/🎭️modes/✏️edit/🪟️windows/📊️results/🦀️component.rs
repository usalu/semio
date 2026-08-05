//! 📊️ DIN 4108 play app — the results window: every computed compliance check, one line each.

use crate::artifacts::din4108::engine::Din4108Family;
use crate::core::NormHost;
use semio_framework_plugin::{LocalizedLabel, UiNode, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_RESULTS: &str = "norm-din4108-results";
pub const BODY_RESULTS: &str = "norm.din4108.play.results";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::apps::din4108::create_din4108_app`.
pub fn definition() -> WindowKindDefinition {
    crate::core::app::window_definition(WINDOW_RESULTS, LocalizedLabel::native("Results", "Ergebnisse"), BODY_RESULTS, "bar-chart-3")
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(host: &NormHost<Din4108Family>) -> UiNode {
    crate::core::app::render_report(host.report())
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::din4108::testkit;

    #[test]
    fn definition_declares_this_windows_body_key() {
        assert_eq!(definition().body_key, BODY_RESULTS);
        assert_eq!(definition().id, WINDOW_RESULTS);
    }

    #[test]
    fn renders_the_computed_checks() {
        let mut app = testkit::new_app();
        let rendered = testkit::render(&mut app, BODY_RESULTS);
        assert!(!rendered.contains("No checks computed."), "the default document must compute at least one check: {rendered}");
    }
}
//#endregion 🧪️Tests
