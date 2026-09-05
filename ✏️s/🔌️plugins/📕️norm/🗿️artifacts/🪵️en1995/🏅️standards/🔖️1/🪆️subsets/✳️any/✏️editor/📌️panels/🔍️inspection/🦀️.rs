//! 🔍️ EN 1995 play app panel — the inspection tab: one computed check in full, chosen by the config's
//! `selected_check_index` (the only view state a norm app carries).

use crate::document::NormHost;
use crate::editor::en1995::En1995Family;
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, UiNode, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

//#region 🔖️Constants
pub const BODY_INSPECTION: &str = "norm.en1995.play.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    crate::app_surface::panel_definition(FRAMEWORK_PANEL_TAB_INSPECTION_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"), PanelGroup::Details, BODY_INSPECTION)
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(host: &NormHost<En1995Family>, selected_check_index: Option<u32>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    crate::app_surface::render_inspection(host.report(), selected_check_index)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::en1995::testkit;

    #[semio_framework_async_macros::async_test]
    fn definition_binds_the_framework_inspection_tab_to_this_body_key() {
        assert_eq!(definition().body_key.as_deref(), Some(BODY_INSPECTION));
        assert_eq!(definition().id(), FRAMEWORK_PANEL_TAB_INSPECTION_ID);
        assert!(matches!(definition().group, PanelGroup::Details));
    }

    /// 👁️ The config-driven pointer: an out-of-range index falls back to the first check, so both
    /// renders agree for a document whose report has fewer rows than the index.
    #[semio_framework_async_macros::async_test]
    fn an_out_of_range_selected_index_falls_back_to_the_first_check() {
        let host = NormHost::<En1995Family>::from_document(crate::artifacts::en1995::En1995Snapshot::default());
        let first = serde_json::to_string(&render(&host, None)).expect("json");
        let clamped = serde_json::to_string(&render(&host, Some(9_999))).expect("json");
        assert_eq!(first, clamped);
    }

    #[semio_framework_async_macros::async_test]
    fn renders_a_single_check() {
        let mut app = testkit::app_with_registry();
        assert!(!testkit::render(&mut app, BODY_INSPECTION).contains("Unknown body"));
    }
}
//#endregion 🧪️Tests
