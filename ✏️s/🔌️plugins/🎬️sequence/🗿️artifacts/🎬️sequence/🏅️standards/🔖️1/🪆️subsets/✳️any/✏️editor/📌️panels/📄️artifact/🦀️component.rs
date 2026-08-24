//! 📄️ Sequence play app panel — the document tree: steps (with control-flow slot nesting) and edges.

use crate::artifacts::sequence::{SequenceFixture, SequenceStep};
use crate::editor::sequence::sequence_action;
use crate::editor::sequence::terminology::SequenceLabels;
use crate::editor::sequence::{control_slots, is_control_kind, SEQUENCE_INTERACTION_STEPS};
use semio_framework_plugin::{
    tree_item_desc, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiControlNode, UiNode, UiPresence, UiToggleNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL,
};
use serde_json::json;

//#region 🔖️Constants
pub const SEQUENCE_PLAY_BODY_DOCUMENT: &str = "sequence.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(SEQUENCE_PLAY_BODY_DOCUMENT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Helpers
/// 🗣️ Localizes a control-flow slot name ("then"/"else"/"body") for tree display; unknown slot names
/// fall back to the raw id as genuine runtime data (never authored UI copy).
async fn slot_label(slot_name: &str, labels: &SequenceLabels) -> Label {
    match slot_name {
        "then" => labels.slot_then.into(),
        "else" => labels.slot_else.into(),
        "body" => labels.slot_body.into(),
        other => Label::data(other),
    }
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: item ids are the SAME canonical raw
/// step ids `SequencePlayApp::interaction_topology` declares for the "steps" domain (and the main
/// node-graph canvas's own `NodeGraphNodeRecord.id`) — the framework stamps this tree's
/// selection/hover presence from that domain (`.interaction_domain`) and prunes stale ids through
/// that same topology, so no per-item click action is declared here anymore (clicks are translated
/// into `interactionSelect` generically)?.
async fn build_step_tree_item(step: &SequenceStep, fixture: &SequenceFixture, labels: &SequenceLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut item = tree_item_desc(step.id.clone(), Label::data(format!("{} ({})", step.id, step.kind)), Some(step.kind.clone()))?;
    if is_control_kind(&step.kind) {
        item.control = Some(UiControlNode::Toggle(UiToggleNode {
            id: format!("sequence-play-document.collapse.{}", step.id),
            icon_id: if step.collapsed { "chevron-right" } else { "chevron-down" }.into(),
            presence: UiPresence::selected(!step.collapsed),
            text: None,
            on_change: sequence_action("setStepCollapsed", Some(json!({ "id": step.id }))),
            menu: None,
        }));
        let slot_items: Vec<UiTreeItemNode> = control_slots(&step.kind)
            .iter()
            .map(|slot_name| {
                let nested: Vec<UiTreeItemNode> = fixture.steps.iter().filter(|entry| entry.slot.as_ref().is_some_and(|slot| slot.owner == step.id && slot.name == *slot_name)).map(|entry| build_step_tree_item(entry, fixture, labels)?).collect();
                UiTreeItemNode {
                    id: format!("sequence-play-document.slot.{}.{}", step.id, slot_name),
                    label: slot_label(slot_name, labels),
                    description: Some(format!("{} {}", step.id, labels.slot.as_str())),
                    icon_id: Some("folder".into()),
                    presence: UiPresence::default(),
                    default_open: Some(true),
                    action: None,
                    actions: None,
                    draggable: None,
                    drag_data: None,
                    items: if nested.is_empty() { None } else { Some(nested) },
                    control: None,
                    dimmed: if step.collapsed { Some(true) } else { None },
                    menu: None,
                }
            })
            .collect();
        if !slot_items.is_empty() {
            item.items = Some(slot_items);
        }
        item.default_open = Some(!step.collapsed);
    }
    item
}
//#endregion 🔖️Helpers

//#region 🔖️Render
pub async fn render(fixture: &SequenceFixture, labels: &SequenceLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let step_items: Vec<UiTreeItemNode> = fixture.steps.iter().filter(|step| step.slot.is_none()).map(|step| build_step_tree_item(step, fixture, labels)?).collect();
    let edge_items: Vec<UiTreeItemNode> = fixture.edges.iter().map(|edge| tree_item_desc(format!("sequence-play-document.edge.{}", edge.id), Label::data(format!("{} → {}", edge.from, edge.to)), Some(edge.id.clone()))?).collect();
    PanelTreeBuilder::new("sequence-play-document")?
        .section_or_placeholder("sequence-play-document.steps", Some(labels.steps.into()), true, step_items, labels.none)?
        .section_or_placeholder("sequence-play-document.edges", Some(labels.flow_edges.into()), false, edge_items, labels.none)?
        .interaction_domain(SEQUENCE_INTERACTION_STEPS)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::sequence::testkit::{new_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn document_lists_steps() {
        let mut app = new_app();
        assert!(render_body(&mut app, SEQUENCE_PLAY_BODY_DOCUMENT).contains("sequence-play-document.steps"));
    }

    #[semio_framework_async_macros::async_test]
    async fn definition_binds_the_framework_document_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
        assert_eq!(definition.body_key.as_deref(), Some(SEQUENCE_PLAY_BODY_DOCUMENT));
    }
}
//#endregion 🧪️Tests
