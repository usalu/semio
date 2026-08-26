//! 📄️ EN 1992 play app panel — the document headline: family, check count, worst utilization, verdict.

use crate::document::NormHost;
use crate::editor::en1992::En1992Family;
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, UiNode, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

//#region 🔖️Constants
pub const BODY_DOCUMENT: &str = "norm.en1992.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    crate::app_surface::panel_definition(FRAMEWORK_PANEL_TAB_ARTIFACT_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"), PanelGroup::Workbench, BODY_DOCUMENT)
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(host: &NormHost<En1992Family>) -> UiNode {
    crate::app_surface::render_summary(host)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::en1992::testkit;

    #[semio_framework_async_macros::async_test]
    fn definition_binds_the_framework_document_tab_to_this_body_key() {
        assert_eq!(definition().body_key.as_deref(), Some(BODY_DOCUMENT));
        assert_eq!(definition().id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
    }

    #[semio_framework_async_macros::async_test]
    fn renders_the_family_headline() {
        let mut app = testkit::new_app();
        assert!(testkit::render(&mut app, BODY_DOCUMENT).contains("checks"));
    }
}
//#endregion 🧪️Tests
