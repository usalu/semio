//! 📊️ VDI 3805 play app — the results window: every computed compliance check, one line each.

use crate::apps::vdi3805::Vdi3805Family;
use crate::document::NormHost;
use semio_framework_plugin::{LocalizedLabel, UiNode, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_RESULTS: &str = "norm-vdi3805-results";
pub const BODY_RESULTS: &str = "norm.vdi3805.play.results";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::apps::vdi3805::create_vdi3805_app`.
pub fn definition() -> WindowKindDefinition {
    crate::app_surface::window_definition(WINDOW_RESULTS, LocalizedLabel::native("Results", "Ergebnisse"), BODY_RESULTS, "bar-chart-3")
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(host: &NormHost<Vdi3805Family>) -> UiNode {
    crate::app_surface::render_report(host.report())
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::vdi3805::testkit;

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
