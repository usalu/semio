//! 🛠️ Process 3d play app panel — the workshop configurator: installed machines (select, remove) plus
//! one section per installed catalog that still has something left to add — a catalog fully installed,
//! or a machine already installed from it, renders no duplicate row here (it stays visible above, in
//! the installed-machines section, where remove lives).

use crate::editor::process3d::iconed_tree_item_with_action;
use crate::editor::process3d::installed_catalogs;
use crate::editor::process3d::process3d_action;
use crate::editor::process3d::terminology::Process3dLabels;
use crate::editor::process3d::{PROCESS3D_GRANULARITY_OBJECT, PROCESS3D_INTERACTION_DOMAIN, PROCESS_3D_PLAY_APP_ID};
use crate::{MachineCatalog, Process3dSnapshot, WorkshopMachine};
use semio_framework_plugin::{tree_item, ActionBinding, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, RowAction, RowActionPlacement, Trigger, TreeWindows, UiAssemblyResult};

//#region 🔖️Constants
pub const PROCESS_3D_PLAY_BODY_WORKSHOP: &str = "process.play.workshop";
pub const PROCESS_3D_PLAY_WORKSHOP_MACHINES: &str = "process3d-play-workshop.machines";
const PROCESS_3D_PLAY_PANEL_WORKSHOP: &str = "workshop";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(PROCESS_3D_PLAY_PANEL_WORKSHOP.into()),
        label: LocalizedLabel::native("Workshop", "Werkstatt"),
        group: PanelGroup::Workbench,
        body_key: Some(PROCESS_3D_PLAY_BODY_WORKSHOP.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn workshop_error(scope: &'static str) -> semio_framework_plugin::PluginAssemblyError {
    semio_framework_plugin::PluginAssemblyError::new("ui.workshop", scope)
}

fn ui_text(value: &str) -> UiAssemblyResult<semio_framework_plugin::UiText> {
    semio_framework_plugin::UiText::try_from_str(value).ok_or_else(|| workshop_error("text"))
}

/// 🛠️ One installed-machine row: `removeWorkshopMachine` stays its own menu row action, selection is
/// the tree's — the row declares the `object` granularity and picks through the tree's single
/// `interactionSelect` binding.
fn machine_row(machine: &WorkshopMachine, labels: &Process3dLabels) -> UiAssemblyResult<BuiltNode> {
    let args = crate::editor::process3d::ui_value_map([("id", crate::editor::process3d::ui_value_text(&machine.id)?)])?;
    let (action, args) = process3d_action("removeWorkshopMachine", Some(args))?;
    let mut row_actions = semio_framework_plugin::UiFixedList::default();
    row_actions
        .try_push(RowAction {
            icon: ui_text("trash")?,
            label: Some(crate::editor::process3d::ui_label(labels.remove_machine.as_str())?),
            action: ActionBinding { trigger: Trigger::Activate, action, args, capability: None },
            placement: RowActionPlacement::Menu,
        })
        .map_err(|_| workshop_error("row-actions"))?;
    let mut item = tree_item(format!("machine:{}", machine.id), crate::editor::process3d::ui_label(&machine.label)?)?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
        props.icon = Some(ui_text(&machine.icon_id)?);
        props.granularity = Some(ui_text(PROCESS3D_GRANULARITY_OBJECT)?);
        props.row_actions = row_actions;
    }
    Ok(item)
}

/// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): installed-machine item ids are
/// `"machine:{id}"` — the SAME canonical `"geometry"` domain target the old `selected_id` used for a
/// machine pick — so `.interaction_domain` binding plus the row's own `granularity` make a click a
/// framework pick; the catalog sections' rows stay install ACTIONS, not domain targets, so they
/// declare no granularity.
///
/// 🪟️ Every section is WINDOWED: the installed-machine section states its full count and the per-
/// catalog install sections author themselves closed, so a workshop with a dozen contributed catalogs
/// costs one viewport of rows on first paint (ticket 26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING).
pub fn render(snapshot: &Process3dSnapshot, contributions_json: &str, labels: &Process3dLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let mut builder = PanelTreeBuilder::new("process3d-play-workshop")?
        .window_section(windows, PROCESS_3D_PLAY_WORKSHOP_MACHINES, Some(crate::editor::process3d::ui_label(labels.machines.as_str())?), true, &snapshot.workshop.machines, |machine| {
            machine_row(machine, labels)
        })?
        .interaction_domain(PROCESS_3D_PLAY_APP_ID, PROCESS3D_INTERACTION_DOMAIN)?;
    let installed_ids: std::collections::BTreeSet<&str> = snapshot.workshop.machines.iter().map(|machine| machine.id.as_str()).collect();
    for catalog in installed_catalogs(contributions_json) {
        let catalog_id = catalog.catalog_id();
        let installable: Vec<_> = catalog.machines().into_iter().filter(|machine| !installed_ids.contains(machine.id.as_str())).collect();
        if installable.is_empty() {
            continue;
        }
        let section_id = format!("process3d-play-workshop.catalog.{catalog_id}");
        builder = builder.window_section(windows, &section_id, Some(crate::editor::process3d::ui_label(catalog.label())?), false, &installable, |machine| {
            let id = format!("process3d-workshop.catalog.{catalog_id}.{}", machine.id);
            let args = crate::editor::process3d::ui_value_map([("catalogId", crate::editor::process3d::ui_value_text(catalog_id)?), ("machineId", crate::editor::process3d::ui_value_text(&machine.id)?)])?;
            iconed_tree_item_with_action(id, &machine.label, &machine.icon_id, process3d_action("addWorkshopMachine", Some(args)))
        })?;
    }
    builder.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
