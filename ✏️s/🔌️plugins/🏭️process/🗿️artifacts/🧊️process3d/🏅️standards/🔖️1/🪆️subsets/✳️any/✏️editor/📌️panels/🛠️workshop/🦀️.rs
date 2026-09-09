//! 🛠️ Process 3d play app panel — the workshop configurator: installed machines (select, remove) plus
//! one section per installed catalog that still has something left to add — a catalog fully installed,
//! or a machine already installed from it, renders no duplicate row here (it stays visible above, in
//! the installed-machines section, where remove lives).

use crate::editor::process3d::iconed_tree_item_with_action;
use crate::editor::process3d::installed_catalogs;
use crate::editor::process3d::process3d_action;
use crate::editor::process3d::terminology::Process3dLabels;
use crate::editor::process3d::PROCESS3D_INTERACTION_DOMAIN;
use crate::{MachineCatalog, Process3dSnapshot};
use semio_framework_plugin::{tree_item, ActionBinding, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, RowAction, RowActionPlacement, Trigger};

//#region 🔖️Constants
pub const PROCESS_3D_PLAY_BODY_WORKSHOP: &str = "process.play.workshop";
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
/// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): installed-machine item ids are
/// `"machine:{id}"` — the SAME canonical `"geometry"` domain target the old `selected_id` used for a
/// machine pick — so `.interaction_domain` binding stamps/prunes this section correctly; the catalog
/// sections stay un-bound (their items are install actions, not domain targets)?.
pub fn render(fixture: &Process3dSnapshot, contributions_json: &str, labels: &Process3dLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut builder = PanelTreeBuilder::new("process3d-play-workshop")?;
    let mut machine_items = semio_framework_plugin::UiFixedList::default();
    for machine in &fixture.workshop.machines {
        let args = crate::editor::process3d::ui_value_map([("id", crate::editor::process3d::ui_value_text(&machine.id)?)])?;
        let (action, args) = process3d_action("removeWorkshopMachine", Some(args))?;
        let mut row_actions = semio_framework_plugin::UiFixedList::default();
        row_actions
            .try_push(RowAction {
                icon: semio_framework_plugin::UiText::try_from_str("trash").ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.workshop.action-icon", "fixed workshop action icon admission failed"))?,
                label: Some(crate::editor::process3d::ui_label(labels.remove_machine.as_str())?),
                action: ActionBinding { trigger: Trigger::Activate, action, args, capability: None },
                placement: RowActionPlacement::Menu,
            })
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.workshop.row-actions", "fixed workshop row action admission failed"))?;
        let mut item = tree_item(format!("machine:{}", machine.id), crate::editor::process3d::ui_label(&machine.label)?)?;
        if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
            props.icon = Some(semio_framework_plugin::UiText::try_from_str(&machine.icon_id).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.workshop.icon", "fixed workshop icon admission failed"))?);
            props.row_actions = row_actions;
        }
        machine_items.try_push(item).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.workshop.machines", "fixed workshop machine admission failed"))?;
    }
    builder = builder.section("process3d-play-workshop.machines", Some(crate::editor::process3d::ui_label(labels.machines.as_str())?), true, machine_items)?.interaction_domain(PROCESS3D_INTERACTION_DOMAIN)?;
    let installed_ids: std::collections::BTreeSet<&str> = fixture.workshop.machines.iter().map(|machine| machine.id.as_str()).collect();
    for catalog in installed_catalogs(contributions_json) {
        let catalog_id = catalog.catalog_id();
        let installable: Vec<_> = catalog.machines().into_iter().filter(|machine| !installed_ids.contains(machine.id.as_str())).collect();
        if installable.is_empty() {
            continue;
        }
        let mut items = semio_framework_plugin::UiFixedList::default();
        for machine in installable {
            let id = format!("process3d-workshop.catalog.{catalog_id}.{}", machine.id);
            let args = crate::editor::process3d::ui_value_map([("catalogId", crate::editor::process3d::ui_value_text(catalog_id)?), ("machineId", crate::editor::process3d::ui_value_text(&machine.id)?)])?;
            let item = iconed_tree_item_with_action(id, &machine.label, &machine.icon_id, process3d_action("addWorkshopMachine", Some(args)))?;
            items.try_push(item).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.workshop.catalogue", "fixed workshop catalogue admission failed"))?;
        }
        builder = builder.section(format!("process3d-play-workshop.catalog.{catalog_id}"), Some(crate::editor::process3d::ui_label(catalog.label())?), false, items)?;
    }
    builder.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
