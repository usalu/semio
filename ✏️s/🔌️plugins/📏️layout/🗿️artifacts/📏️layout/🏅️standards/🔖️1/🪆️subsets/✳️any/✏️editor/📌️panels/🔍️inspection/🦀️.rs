//! 🔍️ Layout play app panel — the inspector: a document summary (was field editors for the current
//! selection; see `render`'s doc comment for why that's gone).

use crate::artifacts::layout::{LayoutSnapshot, LAYOUT_DOCUMENT_SCHEMA};
use crate::editor::layout::config::LayoutConfig;
use crate::editor::layout::terminology::LayoutLabels;
use semio_framework_plugin::{ui_declarative_sections_to_tree, ui_text, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiNode, UiPresence, UiSectionNode, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

//#region 🔖️Constants
pub const LAYOUT_PLAY_BODY_INSPECTION: &str = "layout.play.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(LAYOUT_PLAY_BODY_INSPECTION.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: this used to switch on
/// `config.selected_ids` to show one field-editor group per selected page/frame (name, bounds,
/// fill/stroke, story content, wrap mode, link path). Selection is now framework-owned
/// (`InteractionView`, threaded only into `handle`/`copy_fragment`/`cut_operations`) and
/// `ArtifactApp::render` never gained that parameter, so this panel has no live selection to render
/// against and always falls through to the document summary below — the same gap gis2d's and
/// puzzle3d's inspection panels flag (see this ticket's w3b-summary.md). Not fixed here (framework
/// file, out of this crate's remit).
pub async fn render(doc: &LayoutSnapshot, config: &LayoutConfig, labels: &LayoutLabels) -> UiNode {
    ui_declarative_sections_to_tree(&[UiSectionNode {
        id: "layout-play-inspector.empty".into(),
        label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
        default_open: Some(true),
        children: vec![
            ui_text(Label::data(format!("{}: {}", labels.schema.as_str(), LAYOUT_DOCUMENT_SCHEMA))),
            ui_text(Label::data(format!("{}: {}", labels.name.as_str(), doc.name))),
            ui_text(Label::data(format!("{}: {}", labels.pages.as_str(), doc.pages.len()))),
            ui_text(Label::data(format!("{}: {}", labels.active_page.as_str(), config.active_page_id))),
        ],
        presence: UiPresence::default(),
        menu: None,
    }])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::layout::testkit::{layout_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn the_inspector_always_summarises_the_document() {
        let mut app = layout_app();
        let json = render_body(&mut app, LAYOUT_PLAY_BODY_INSPECTION);
        assert!(json.contains(LAYOUT_DOCUMENT_SCHEMA));
        assert!(json.contains("page-1"));
    }

    #[semio_framework_async_macros::async_test]
    async fn definition_binds_the_framework_inspection_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_INSPECTION_ID);
        assert_eq!(definition.body_key.as_deref(), Some(LAYOUT_PLAY_BODY_INSPECTION));
    }
}
//#endregion 🧪️Tests
