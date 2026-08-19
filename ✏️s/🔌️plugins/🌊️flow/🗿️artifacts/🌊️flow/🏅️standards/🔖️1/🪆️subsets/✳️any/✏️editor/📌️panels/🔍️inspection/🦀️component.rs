//! 🔍️ Flow play app panel — the active selection's inspector (name, per-kind fields).

use crate::editor::flow::terminology::FlowPlayLabels;
use semio_framework_plugin::{
    ui_declarative_sections_to_tree, ui_text, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiNode, UiPresence, UiSectionNode, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};

//#region 🔖️Constants
pub const FLOW_PLAY_BODY_INSPECTOR: &str = "flow.play.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(FLOW_PLAY_BODY_INSPECTOR.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the per-widget field groups (rename,
/// slider value, note text — driven by which widgets are selected) used to read `FlowConfig`; the
/// "graph" domain's selection is framework-owned `InteractionState` now, and `ArtifactApp::render` is
/// not threaded an `InteractionView` this wave (only `handle`/`copy_fragment`/`cut_operations` are, see
/// `FlowPlayApp::handle`) — dropped rather than shown stale, mirroring lowpoly's identical
/// `render`/status-line note for the exact same gap. Peer/self selection surfaces generically off the
/// declared domain regardless.
pub async fn render(labels: &FlowPlayLabels) -> UiNode {
    ui_declarative_sections_to_tree(&[UiSectionNode {
        presence: UiPresence::default(),
        id: "flow-play-inspector.empty".into(),
        label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
        default_open: Some(true),
        children: vec![ui_text(labels.no_selection)],
        menu: None,
    }])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::flow::testkit::{flow_app, render as render_body};
    use crate::editor::flow::FLOW_PLAY_BODY_INSPECTOR as BODY_INSPECTOR;

    #[test]
    async fn empty_inspector_no_longer_shows_canvas_settings() {
        let mut app = flow_app();
        let json = render_body(&mut app, BODY_INSPECTOR);
        assert!(!json.contains("flow-play-inspector.lod-mode"));
        assert!(json.contains("flow-play-inspector.empty"));
    }

    #[test]
    async fn definition_binds_the_framework_inspection_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_INSPECTION_ID);
        assert_eq!(definition.body_key.as_deref(), Some(FLOW_PLAY_BODY_INSPECTOR));
    }
}
//#endregion 🧪️Tests
