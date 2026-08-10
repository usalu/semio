//! 📄️ Process 3d play app panel — the document tree: stock + ordered process steps.

use crate::apps::process3d::config::Process3dConfig;
use crate::apps::process3d::process3d_action;
use crate::apps::process3d::terminology::{process3d_measure_icon, Process3dLabels};
use crate::artifacts::process3d::Process3dSnapshot;
use semio_framework_plugin::{
    Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiPresence, UiTreeActionPlacement, UiTreeItemAction, UiTreeItemNode, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL,
};
use serde_json::json;

//#region 🔖️Constants
pub const PROCESS_3D_PLAY_BODY_DOCUMENT: &str = "process.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(PROCESS_3D_PLAY_BODY_DOCUMENT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(fixture: &Process3dSnapshot, cfg: &Process3dConfig, labels: &Process3dLabels) -> UiNode {
    let stock = &fixture.stock;
    let stock_item = UiTreeItemNode {
        icon_id: Some("box".into()),
        presence: UiPresence::selected(cfg.selected_id.as_deref() == Some(stock.id.as_str())),
        action: Some(process3d_action("setSelection", Some(json!({ "id": stock.id })))),
        menu: None,
        ..UiTreeItemNode::base(stock.id.clone(), Label::data(stock.label.clone()))
    };
    let cursor = fixture.resolved_up_to.unwrap_or(fixture.steps.len());
    let step_items: Vec<UiTreeItemNode> = fixture
        .steps
        .iter()
        .enumerate()
        .map(|(index, step)| UiTreeItemNode {
            description: if index >= cursor { Some("pending".into()) } else { None },
            icon_id: Some(process3d_measure_icon(&step.measure).into()),
            presence: UiPresence::selected(cfg.selected_id.as_deref() == Some(step.id.as_str())),
            action: Some(process3d_action("setSelection", Some(json!({ "id": step.id })))),
            hover_action: Some(process3d_action("setHover", Some(json!({ "id": step.id })))),
            unhover_action: Some(process3d_action("setHover", None)),
            actions: Some(vec![
                UiTreeItemAction {
                    icon_id: if step.enabled { "eye".into() } else { "eye-off".into() },
                    label: Some(labels.enabled.into()),
                    action: process3d_action("setStepEnabled", Some(json!({ "id": step.id, "enabled": !step.enabled }))),
                    placement: Some(UiTreeActionPlacement::Row),
                },
                UiTreeItemAction { icon_id: "trash".into(), label: Some(labels.remove.into()), action: process3d_action("removeStep", Some(json!({ "id": step.id }))), placement: Some(UiTreeActionPlacement::Menu) },
            ]),
            dimmed: Some(!step.enabled),
            menu: None,
            ..UiTreeItemNode::base(step.id.clone(), Label::data(step.label.clone()))
        })
        .collect();
    PanelTreeBuilder::new("process3d-play-document").section("process3d-play-document.stock", Some(labels.stock.into()), true, vec![stock_item]).section("process3d-play-document.steps", Some(labels.steps.into()), true, step_items).build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::process3d::testkit;

    #[test]
    fn definition_binds_the_framework_document_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
        assert_eq!(definition.body_key.as_deref(), Some(PROCESS_3D_PLAY_BODY_DOCUMENT));
    }

    #[test]
    fn document_panel_lists_stock_and_steps() {
        let mut app = testkit::app();
        let rendered = testkit::render(&mut app, PROCESS_3D_PLAY_BODY_DOCUMENT);
        assert!(rendered.contains("process3d-play-document.stock"));
        assert!(rendered.contains("process3d-play-document.steps"));
    }
}
//#endregion 🧪️Tests
