//! 🔍️ DIN 4108 play app panel — the inspection tab: one computed check in full, chosen by the config's
//! `selected_check_index` (the only view state a norm app carries).

use crate::artifacts::din4108::engine::Din4108Family;
use crate::core::NormHost;
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, UiNode, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

//#region 🔖️Constants
pub const BODY_INSPECTION: &str = "norm.din4108.play.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    crate::core::app::panel_definition(FRAMEWORK_PANEL_TAB_INSPECTION_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"), PanelGroup::Details, BODY_INSPECTION)
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(host: &NormHost<Din4108Family>, selected_check_index: Option<u32>) -> UiNode {
    crate::core::app::render_inspection(host.report(), selected_check_index)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::din4108::testkit;

    #[test]
    fn definition_binds_the_framework_inspection_tab_to_this_body_key() {
        assert_eq!(definition().body_key.as_deref(), Some(BODY_INSPECTION));
        assert_eq!(definition().id(), FRAMEWORK_PANEL_TAB_INSPECTION_ID);
        assert!(matches!(definition().group, PanelGroup::Details));
    }

    /// 👁️ The config-driven pointer: an out-of-range index falls back to the first check, so both
    /// renders agree for a document whose report has fewer rows than the index.
    #[test]
    fn an_out_of_range_selected_index_falls_back_to_the_first_check() {
        let host = NormHost::<Din4108Family>::from_document(crate::artifacts::din4108::Document::default());
        let first = serde_json::to_string(&render(&host, None)).expect("json");
        let clamped = serde_json::to_string(&render(&host, Some(9_999))).expect("json");
        assert_eq!(first, clamped);
    }

    #[test]
    fn renders_a_single_check() {
        let mut app = testkit::new_app();
        assert!(!testkit::render(&mut app, BODY_INSPECTION).contains("Unknown body"));
    }
}
//#endregion 🧪️Tests
