//! 📄️ Process 3d play app panel — the document tree: stock + ordered process steps.

use crate::artifacts::process3d::Process3dSnapshot;
use crate::editor::process3d::process3d_action;
use crate::editor::process3d::terminology::{process3d_measure_icon, Process3dLabels};
use crate::editor::process3d::PROCESS3D_INTERACTION_DOMAIN;
use semio_framework_plugin::{
    tree_item, ActionBinding, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, RowAction, RowActionPlacement, Trigger, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL,
};

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
/// 🌉️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM wave 4: `fixture.steps` is a composed
/// `s.stdio.semio.flow` CHILD HANDLE now, with no resolvable content without a `LinkResolver` (see
/// `ProcessWorkingScene`'s doc comment) — the steps section renders empty, a documented gap
/// matching `📐️cad`'s own per-pane panels.
///
/// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): item ids (`fixture.stock_id`, each
/// step id) are the SAME canonical targets the framework-owned `"geometry"` interaction domain
/// selects — the tree binds `.interaction_domain` and stamps no `.selected()?`/`.highlighted()?`
/// itself; the framework's post-render pass overwrites item presence from live selection/hover, and
/// clicks translate into `interactionSelect` generically (mirrors `🧱️block`'s `📌️panels/📄️artifact`).
pub fn render(fixture: &Process3dSnapshot, labels: &Process3dLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut stock_item = tree_item(&fixture.stock_id, crate::editor::process3d::ui_label(&fixture.stock_label)?)?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut stock_item.component {
        props.icon = Some(semio_framework_plugin::UiText::try_from_str("box").ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.document.icon", "fixed document icon admission failed"))?);
    }
    let scene = crate::artifacts::process3d::process_working_scene_from_snapshot(fixture);
    let cursor = fixture.resolved_up_to.unwrap_or(scene.steps.len());
    let mut step_items = semio_framework_plugin::UiFixedList::default();
    for (index, step) in scene.steps.iter().enumerate() {
        let mut item = tree_item(&step.id, crate::editor::process3d::ui_label(&step.label)?)?;
        let icon = semio_framework_plugin::UiText::try_from_str(process3d_measure_icon(&step.measure))
            .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.document.icon", "fixed document icon admission failed"))?;
        let enabled_args = crate::editor::process3d::ui_value_map([
            ("enabled", crate::editor::process3d::ui_value_bool(!step.enabled)),
            ("id", crate::editor::process3d::ui_value_text(&step.id)?),
        ])?;
        let (enabled_action, enabled_args) = process3d_action("setStepEnabled", Some(enabled_args))?;
        let remove_args = crate::editor::process3d::ui_value_map([("id", crate::editor::process3d::ui_value_text(&step.id)?)])?;
        let (remove_action, remove_args) = process3d_action("removeStep", Some(remove_args))?;
        let mut row_actions = semio_framework_plugin::UiFixedList::default();
        row_actions
            .try_push(RowAction {
                icon: semio_framework_plugin::UiText::try_from_str(if step.enabled { "eye" } else { "eye-off" })
                    .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.document.action-icon", "fixed row action icon admission failed"))?,
                label: Some(crate::editor::process3d::ui_label(labels.enabled.as_str())?),
                action: ActionBinding { trigger: Trigger::Activate, action: enabled_action, args: enabled_args, capability: None },
                placement: RowActionPlacement::Row,
            })
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.document.row-actions", "fixed row action admission failed"))?;
        row_actions
            .try_push(RowAction {
                icon: semio_framework_plugin::UiText::try_from_str("trash")
                    .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.document.action-icon", "fixed row action icon admission failed"))?,
                label: Some(crate::editor::process3d::ui_label(labels.remove.as_str())?),
                action: ActionBinding { trigger: Trigger::Activate, action: remove_action, args: remove_args, capability: None },
                placement: RowActionPlacement::Menu,
            })
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.document.row-actions", "fixed row action admission failed"))?;
        if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
            props.description = if index >= cursor {
                Some(semio_framework_plugin::UiText::try_from_str("pending").ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.document.description", "fixed document description admission failed"))?)
            } else {
                None
            };
            props.icon = Some(icon);
            props.dimmed = Some(!step.enabled);
            props.row_actions = row_actions;
        }
        step_items.try_push(item).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.document.steps", "fixed step list admission failed"))?;
    }
    let stock_items = crate::editor::process3d::ui_node_list([Ok(stock_item)])?;
    PanelTreeBuilder::new("process3d-play-document")?
        .section("process3d-play-document.stock", Some(crate::editor::process3d::ui_label(labels.stock.as_str())?), true, stock_items)?
        .section("process3d-play-document.steps", Some(crate::editor::process3d::ui_label(labels.steps.as_str())?), true, step_items)?
        .interaction_domain(PROCESS3D_INTERACTION_DOMAIN)?
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
