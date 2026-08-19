//! 📚️ ISO 16757 play app panel — the catalogue tab.
//!
//! 📌️ The catalogue surface is a headline placeholder today (no norm family ships a browsable clause
//! catalogue yet); the tab exists so the framework's workbench group has this app's slot reserved and
//! the body key resolves instead of falling through to the unknown-body text node.

use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, UiNode, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

//#region 🔖️Constants
pub const BODY_CATALOGUE: &str = "norm.iso16757.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    crate::app_surface::panel_definition(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"), PanelGroup::Workbench, BODY_CATALOGUE)
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub async fn render() -> UiNode {
    crate::app_surface::render_catalogue(crate::editor::iso16757::LABEL)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::iso16757::testkit;

    #[test]
    async fn definition_binds_the_framework_catalogue_tab_to_this_body_key() {
        assert_eq!(definition().body_key.as_deref(), Some(BODY_CATALOGUE));
        assert_eq!(definition().id(), FRAMEWORK_PANEL_TAB_CATALOGUE_ID);
    }

    #[test]
    async fn renders_this_standards_catalogue_headline() {
        let mut app = testkit::new_app();
        assert!(testkit::render(&mut app, BODY_CATALOGUE).contains("catalogue"));
    }
}
//#endregion 🧪️Tests
