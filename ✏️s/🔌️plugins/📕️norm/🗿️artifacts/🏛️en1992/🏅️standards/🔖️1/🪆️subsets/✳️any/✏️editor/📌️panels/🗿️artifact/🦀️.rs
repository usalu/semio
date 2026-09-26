//! 📄️ EN 1992 play app panel — the document headline: family, check count, worst utilization, verdict.

use crate::document::NormHost;
use crate::editor::en1992::En1992Family;
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

//#region 🔖️Constants
pub const BODY_ARTIFACT: &str = "norm.en1992.play.artifact";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    crate::app_surface::panel_definition(FRAMEWORK_PANEL_TAB_ARTIFACT_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Artefakt"), PanelGroup::Workbench, BODY_ARTIFACT)
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(host: &NormHost<En1992Family>, locale: semio_framework_plugin::Locale) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    crate::app_surface::render_summary(host, locale)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
