//! 📄️ Sequence play app panel — the document tree: steps (with control-flow slot nesting) and edges.

use crate::artifacts::sequence::{SequenceFixture, SequenceStep};
use crate::editor::sequence::{sequence_action, ui_node_list, ui_value_map, ui_value_text};
use crate::editor::sequence::terminology::SequenceLabels;
use crate::editor::sequence::{control_slots, is_control_kind, SEQUENCE_INTERACTION_STEPS};
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase, HasChildren, Trigger};
use semio_framework_plugin::{tree_item_desc, BuiltNode, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiFixedList, UiText, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};
use semio_framework_ui_contract as ui;

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
fn slot_label(slot_name: &str, labels: &SequenceLabels) -> Label {
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
fn build_step_tree_item(step: &SequenceStep, fixture: &SequenceFixture, labels: &SequenceLabels) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let mut builder = ui::tree_item(Label::data(format!("{} ({})", step.id, step.kind)))
        .try_id(&step.id)
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "sequence step id admission failed"))?
        .description(UiText::try_from_str(&step.kind).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "sequence step kind admission failed"))?);
    if is_control_kind(&step.kind) {
        let mut children = UiFixedList::<BuiltNode>::default();
        let action_args = ui_value_map([("id", ui_value_text(&step.id)?)])?;
        let (action, args) = sequence_action("setStepCollapsed", Some(action_args))?;
        let toggle = ui::toggle(!step.collapsed)
            .icon(UiText::try_from_str(if step.collapsed { "chevron-right" } else { "chevron-down" }).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "sequence collapse icon admission failed"))?)
            .try_id(format!("sequence-play-document.collapse.{}", step.id))
            .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "sequence collapse id admission failed"))?;
        let toggle = match args {
            Some(args) => toggle.try_on_with(Trigger::Change, action, args),
            None => toggle.try_on(Trigger::Change, action),
        }
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "sequence collapse action admission failed"))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "sequence collapse control admission failed"))?;
        children.try_push(toggle).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "sequence collapse child admission failed"))?;
        for slot_name in control_slots(&step.kind) {
            let nested = ui_node_list(fixture.steps.iter().filter(|entry| entry.slot.as_ref().is_some_and(|slot| slot.owner == step.id && slot.name == *slot_name)).map(|entry| build_step_tree_item(entry, fixture, labels)))?;
            let slot = ui::tree_item(slot_label(slot_name, labels))
                .try_id(format!("sequence-play-document.slot.{}.{}", step.id, slot_name))
                .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "sequence slot id admission failed"))?
                .description(UiText::try_from_string(format!("{} {}", step.id, labels.slot.as_str())).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "sequence slot description admission failed"))?)
                .icon(UiText::try_from_str("folder").ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "sequence slot icon admission failed"))?)
                .default_open(true)
                .dimmed(step.collapsed)
                .try_children(nested)
                .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "sequence nested step admission failed"))?
                .try_build()
                .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "sequence slot row admission failed"))?;
            children.try_push(slot).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "sequence slot list admission failed"))?;
        }
        builder = builder.default_open(!step.collapsed).try_children(children).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "sequence control children admission failed"))?;
    }
    builder.try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "sequence step row admission failed"))
}
//#endregion 🔖️Helpers

//#region 🔖️Render
pub async fn render(fixture: &SequenceFixture, labels: &SequenceLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let step_items = ui_node_list(fixture.steps.iter().filter(|step| step.slot.is_none()).map(|step| build_step_tree_item(step, fixture, labels)))?;
    let edge_items = ui_node_list(fixture.edges.iter().map(|edge| tree_item_desc(format!("sequence-play-document.edge.{}", edge.id), Label::data(format!("{} → {}", edge.from, edge.to)), Some(edge.id.clone()))))?;
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
