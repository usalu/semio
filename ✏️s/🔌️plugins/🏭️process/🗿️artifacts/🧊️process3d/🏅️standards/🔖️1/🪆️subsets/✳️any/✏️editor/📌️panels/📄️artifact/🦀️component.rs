//! 📄️ Process 3d play app panel — the document tree: stock + ordered process steps.

use crate::editor::process3d::process3d_action;
use crate::editor::process3d::terminology::{process3d_measure_icon, Process3dLabels};
use crate::editor::process3d::PROCESS3D_INTERACTION_DOMAIN;
use crate::artifacts::process3d::Process3dSnapshot;
use semio_framework_plugin::{
    Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeActionPlacement, UiTreeItemAction, UiTreeItemNode, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL,
};
use serde_json::json;

//#region 🔖️Constants
pub const PROCESS_3D_PLAY_BODY_DOCUMENT: &str = "process.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
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
/// 🌉️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM wave 4: `fixture.steps` is a composed
/// `s.stdio.semio.flow` CHILD HANDLE now, with no resolvable content without a `LinkResolver` (see
/// `ProcessWorkingScene`'s doc comment) — the steps section renders empty, a documented gap
/// matching `📐️cad`'s own per-pane panels.
///
/// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): item ids (`fixture.stock_id`, each
/// step id) are the SAME canonical targets the framework-owned `"geometry"` interaction domain
/// selects — the tree binds `.interaction_domain` and stamps no `.selected()`/`.highlighted()`
/// itself; the framework's post-render pass overwrites item presence from live selection/hover, and
/// clicks translate into `interactionSelect` generically (mirrors `🧱️block`'s `📌️panels/📄️artifact`).
pub async fn render(fixture: &Process3dSnapshot, labels: &Process3dLabels) -> UiNode {
    let stock_item = UiTreeItemNode { icon_id: Some("box".into()), menu: None, ..UiTreeItemNode::base(fixture.stock_id.clone(), Label::data(fixture.stock_label.clone())) };
    let scene = crate::artifacts::process3d::process_working_scene_from_snapshot(fixture);
    let cursor = fixture.resolved_up_to.unwrap_or(scene.steps.len());
    let step_items: Vec<UiTreeItemNode> = scene
        .steps
        .iter()
        .enumerate()
        .map(|(index, step)| UiTreeItemNode {
            description: if index >= cursor { Some("pending".into()) } else { None },
            icon_id: Some(process3d_measure_icon(&step.measure).into()),
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
    PanelTreeBuilder::new("process3d-play-document")
        .section("process3d-play-document.stock", Some(labels.stock.into()), true, vec![stock_item])
        .section("process3d-play-document.steps", Some(labels.steps.into()), true, step_items)
        .interaction_domain(PROCESS3D_INTERACTION_DOMAIN)
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::process3d::testkit;

    #[semio_framework_async_macros::async_test]
    async fn definition_binds_the_framework_document_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
        assert_eq!(definition.body_key.as_deref(), Some(PROCESS_3D_PLAY_BODY_DOCUMENT));
    }

    #[semio_framework_async_macros::async_test]
    async fn document_panel_lists_stock_and_steps() {
        let mut app = testkit::app();
        let rendered = testkit::render(&mut app, PROCESS_3D_PLAY_BODY_DOCUMENT);
        assert!(rendered.contains("process3d-play-document.stock"));
        assert!(rendered.contains("process3d-play-document.steps"));
    }
}
//#endregion 🧪️Tests
