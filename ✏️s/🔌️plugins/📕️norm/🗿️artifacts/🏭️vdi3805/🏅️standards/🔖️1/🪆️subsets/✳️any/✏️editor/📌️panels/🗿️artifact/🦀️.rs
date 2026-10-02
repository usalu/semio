//! 📄️ VDI 3805 play app panel — the document headline: family, check count, worst utilization, verdict.

use crate::document::NormHost;
use crate::editor::vdi3805::Vdi3805Family;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::PanelGroup;
use semio_framework_plugin::PanelTabDefinition;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_ARTIFACT_ID;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL;

//#region 🔖️Constants
pub const BODY_ARTIFACT: &str = "norm.vdi3805.play.artifact";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    crate::app_surface::panel_definition(FRAMEWORK_PANEL_TAB_ARTIFACT_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Artefakt"), PanelGroup::Workbench, BODY_ARTIFACT)
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(host: &NormHost<Vdi3805Family>, locale: semio_framework_ui_locale::Locale) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    crate::app_surface::render_summary(host, locale)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
