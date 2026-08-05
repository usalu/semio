//! 📄️ EN 1992 play app panel — the document headline: family, check count, worst utilization, verdict.

use crate::artifacts::en1992::engine::En1992Family;
use crate::core::NormHost;
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, UiNode, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL};

//#region 🔖️Constants
pub const BODY_DOCUMENT: &str = "norm.en1992.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    crate::core::app::panel_definition(FRAMEWORK_PANEL_TAB_DOCUMENT_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, "Dokument"), PanelGroup::Workbench, BODY_DOCUMENT)
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(host: &NormHost<En1992Family>) -> UiNode {
    crate::core::app::render_summary(host)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::en1992::testkit;

    #[test]
    fn definition_binds_the_framework_document_tab_to_this_body_key() {
        assert_eq!(definition().body_key.as_deref(), Some(BODY_DOCUMENT));
        assert_eq!(definition().id(), FRAMEWORK_PANEL_TAB_DOCUMENT_ID);
    }

    #[test]
    fn renders_the_family_headline() {
        let mut app = testkit::new_app();
        assert!(testkit::render(&mut app, BODY_DOCUMENT).contains("checks"));
    }
}
//#endregion 🧪️Tests
