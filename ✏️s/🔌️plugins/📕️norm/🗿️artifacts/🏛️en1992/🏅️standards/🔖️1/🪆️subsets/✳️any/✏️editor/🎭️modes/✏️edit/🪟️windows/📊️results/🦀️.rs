//! 📊️ EN 1992 play app — the results window: every computed compliance check, one line each.

use crate::document::NormHost;
use crate::editor::en1992::En1992Family;
use semio_framework_plugin::{LocalizedLabel, UiNode, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_RESULTS: &str = "norm-en1992-results";
pub const BODY_RESULTS: &str = "norm.en1992.play.results";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::en1992::create_en1992_app`.
pub fn definition() -> WindowKindDefinition {
    crate::app_surface::window_definition(WINDOW_RESULTS, LocalizedLabel::native("Results", "Ergebnisse"), BODY_RESULTS, "bar-chart-3")
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(host: &NormHost<En1992Family>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    crate::app_surface::render_report(host.report())
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::en1992::testkit;

    #[semio_framework_async_macros::async_test]
    fn definition_declares_this_windows_body_key() {
        assert_eq!(definition().body_key, BODY_RESULTS);
        assert_eq!(definition().id, WINDOW_RESULTS);
    }

    #[semio_framework_async_macros::async_test]
    fn renders_the_computed_checks() {
        let mut app = testkit::new_app();
        let rendered = testkit::render(&mut app, BODY_RESULTS);
        assert!(!rendered.contains("No checks computed."), "the default document must compute at least one check: {rendered}");
    }
}
//#endregion 🧪️Tests
