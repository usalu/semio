//! 📄️ Process 3d play app panel — the document tree: stock + ordered process steps.

use crate::editor::process3d::process3d_action;
use crate::editor::process3d::terminology::{process3d_measure_icon, Process3dLabels};
use crate::editor::process3d::{PROCESS3D_GRANULARITY_OBJECT, PROCESS3D_INTERACTION_DOMAIN, PROCESS_3D_PLAY_APP_ID};
use crate::{Process3dSnapshot, ProcessStep};
use semio_framework_plugin::{tree_item, ActionBinding, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, RowAction, RowActionPlacement, Trigger, TreeWindows, UiAssemblyResult, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

//#region 🔖️Constants
pub const PROCESS_3D_PLAY_BODY_ARTIFACT: &str = "process.play.artifact";
pub const PROCESS_3D_PLAY_DOCUMENT_STOCK: &str = "process3d-play-document.stock";
pub const PROCESS_3D_PLAY_DOCUMENT_STEPS: &str = "process3d-play-document.steps";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Artefakt"),
        group: PanelGroup::Workbench,
        body_key: Some(PROCESS_3D_PLAY_BODY_ARTIFACT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn document_error(scope: &'static str) -> semio_framework_plugin::PluginAssemblyError {
    semio_framework_plugin::PluginAssemblyError::new("ui.document", scope)
}

fn ui_text(value: &str) -> UiAssemblyResult<semio_framework_plugin::UiText> {
    semio_framework_plugin::UiText::try_from_str(value).ok_or_else(|| document_error("text"))
}

/// 🪵️ The stock row — the composition identity, always the document's first target.
fn stock_row(snapshot: &Process3dSnapshot) -> UiAssemblyResult<BuiltNode> {
    let mut item = tree_item(&snapshot.stock_id, crate::editor::process3d::ui_label(&snapshot.stock_label)?)?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
        props.icon = Some(ui_text("box")?);
        props.granularity = Some(ui_text(PROCESS3D_GRANULARITY_OBJECT)?);
    }
    Ok(item)
}

/// 🎞️ One step row: its own verbs stay ROW ACTIONS (the eye toggles `setStepEnabled`, the menu's
/// trash dispatches `removeStep`), while selection is the tree's — the row declares its granularity
/// and the tree carries the single `interactionSelect` binding both it and the canvas pick through.
fn step_row(index: usize, step: &ProcessStep, cursor: usize, labels: &Process3dLabels) -> UiAssemblyResult<BuiltNode> {
    let mut item = tree_item(&step.id, crate::editor::process3d::ui_label(&step.label)?)?;
    let enabled_args = crate::editor::process3d::ui_value_map([("enabled", crate::editor::process3d::ui_value_bool(!step.enabled)), ("id", crate::editor::process3d::ui_value_text(&step.id)?)])?;
    let (enabled_action, enabled_args) = process3d_action("setStepEnabled", Some(enabled_args))?;
    let remove_args = crate::editor::process3d::ui_value_map([("id", crate::editor::process3d::ui_value_text(&step.id)?)])?;
    let (remove_action, remove_args) = process3d_action("removeStep", Some(remove_args))?;
    let mut row_actions = semio_framework_plugin::UiFixedList::default();
    row_actions
        .try_push(RowAction {
            icon: ui_text(if step.enabled { "eye" } else { "eye-off" })?,
            label: Some(crate::editor::process3d::ui_label(labels.enabled.as_str())?),
            action: ActionBinding { trigger: Trigger::Activate, action: enabled_action, args: enabled_args, capability: None },
            placement: RowActionPlacement::Row,
        })
        .map_err(|_| document_error("row-actions"))?;
    row_actions
        .try_push(RowAction {
            icon: ui_text("trash")?,
            label: Some(crate::editor::process3d::ui_label(labels.remove.as_str())?),
            action: ActionBinding { trigger: Trigger::Activate, action: remove_action, args: remove_args, capability: None },
            placement: RowActionPlacement::Menu,
        })
        .map_err(|_| document_error("row-actions"))?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
        props.description = if index >= cursor { Some(ui_text("pending")?) } else { None };
        props.icon = Some(ui_text(process3d_measure_icon(&step.measure))?);
        props.dimmed = Some(!step.enabled);
        props.granularity = Some(ui_text(PROCESS3D_GRANULARITY_OBJECT)?);
        props.row_actions = row_actions;
    }
    Ok(item)
}

/// 📄️ Renders the document tree: `snapshot.stock_id`/`stock_label` (composition identity, always
/// authoritative) plus the ordered step timeline read straight off `snapshot.step_payloads` — the
/// snapshot's own inline, authoritative record of the process steps since ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4 (`stock_solid`/`steps` stay composed-child
/// HANDLES with no resolvable content; the payloads are what's real, see `Process3dSnapshot`'s doc
/// comment).
///
/// 🪟️ Both sections are WINDOWED: each states its full `total` and materialises only the rows the
/// host's viewport asked for, so a hundred-step timeline scrolls rather than rendering unbounded
/// (ticket 26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING).
///
/// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): item ids (`snapshot.stock_id`, each
/// step id) are the SAME canonical targets the framework-owned `"geometry"` interaction domain
/// selects — the tree binds `.interaction_domain` and each row declares its `granularity`, so a click
/// becomes an `interactionSelect` with no per-row argument map (mirrors `🧱️block`'s `📌️panels/🗿️artifact`).
pub fn render(snapshot: &Process3dSnapshot, labels: &Process3dLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let cursor = snapshot.resolved_up_to.unwrap_or(snapshot.step_payloads.len());
    let stock = [()];
    let steps: Vec<(usize, &ProcessStep)> = snapshot.step_payloads.iter().enumerate().collect();
    PanelTreeBuilder::new("process3d-play-document")?
        .window_section(windows, PROCESS_3D_PLAY_DOCUMENT_STOCK, Some(crate::editor::process3d::ui_label(labels.stock.as_str())?), true, &stock, |_| stock_row(snapshot))?
        .window_section(windows, PROCESS_3D_PLAY_DOCUMENT_STEPS, Some(crate::editor::process3d::ui_label(labels.steps.as_str())?), true, &steps, |(index, step)| step_row(*index, step, cursor, labels))?
        .interaction_domain(PROCESS_3D_PLAY_APP_ID, PROCESS3D_INTERACTION_DOMAIN)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
